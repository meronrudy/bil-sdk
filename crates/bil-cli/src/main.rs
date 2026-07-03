use bil_core::AssuranceLevel;
use bil_mock::{generate_bank_branch_mock, BankBranchSyntheticConfig};
use clap::{Parser, Subcommand};
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a demo workflow
    Demo {
        #[arg(short, long, default_value = "bank_branch")]
        profile: String,
    },
    /// Check environment capabilities
    Doctor,
    /// Generate a synthetic workflow
    Mock {
        profile: String,
        #[arg(long)]
        seed: Option<u64>,
        #[arg(long)]
        out: Option<String>,
    },
    /// Compile a MIR graph
    Build {
        workflow_file: String,
        #[arg(long)]
        out: Option<String>,
    },
    /// Issue a receipt-backed artifact
    Issue {
        #[arg(long)]
        input: String,
        #[arg(long)]
        out: String,
    },
    /// Verify a receipt
    Verify {
        #[arg(long)]
        receipt: String,
        #[arg(long)]
        out: Option<String>,
        #[arg(long)]
        pretty: bool,
    },
    /// Explain verification findings
    Explain {
        #[arg(long)]
        receipt: String,
        #[arg(long)]
        report: Option<String>,
        #[arg(long)]
        out: Option<String>,
    },
    /// Run conformance tests
    Conformance {
        group: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Demo { profile } => {
            let demo_profile = match profile.as_str() {
                "bank_branch" => bil_sdk::DemoProfile::BankBranch,
                "loan_decision" => bil_sdk::DemoProfile::LoanDecision,
                "ai_assurance" => bil_sdk::DemoProfile::AiAssurance,
                _ => {
                    println!("Unknown profile: {}", profile);
                    return;
                }
            };

            let demo = bil_sdk::Bil::demo(demo_profile).unwrap();
            let receipt = demo.receipt().unwrap();
            let memo = demo.memo().unwrap();

            fs::create_dir_all("artifacts/demo").unwrap();

            let receipt_json = serde_json::to_string_pretty(&receipt).unwrap();
            fs::write("artifacts/demo/ink_receipt.v1.json", receipt_json).unwrap();

            memo.write_markdown("artifacts/demo/assurance_memo.md").unwrap();

            println!("Demo completed successfully.");
            println!("Artifacts written to artifacts/demo/");
        }
        Commands::Doctor => {
            let report = bil_sdk::Bil::doctor().unwrap();
            println!("Doctor Report:");
            println!("  Healthy: {}", report.is_healthy);
            for msg in report.messages {
                println!("  - {}", msg);
            }
        }
        Commands::Mock { profile, seed, out } => {
            if profile == "bank-branch" {
                let config = BankBranchSyntheticConfig {
                    seed: seed.unwrap_or(0),
                    branch_id: "branch-001".to_string(),
                    include_ai_assist: true,
                    include_human_review: true,
                    include_adverse_action: false,
                    signer_level: AssuranceLevel::L0SoftwareDev,
                };
                let graph = generate_bank_branch_mock(&config);
                let json = serde_json::to_string_pretty(&graph).unwrap();
                if let Some(out_path) = out {
                    fs::write(&out_path, json).unwrap();
                    println!("Wrote mock workflow to {}", out_path);
                } else {
                    println!("{}", json);
                }
            } else {
                println!("Unknown profile: {}", profile);
            }
        }
        Commands::Build { workflow_file, out } => {
            let json = fs::read_to_string(&workflow_file).unwrap();
            let graph: bil_mir::BilMirGraph = serde_json::from_str(&json).unwrap();
            let out_json = serde_json::to_string_pretty(&graph).unwrap();
            if let Some(out_path) = out {
                fs::write(&out_path, out_json).unwrap();
                println!("Wrote built MIR to {}", out_path);
            } else {
                println!("{}", out_json);
            }
        }
        Commands::Issue { input, out } => {
            let json = fs::read_to_string(&input).unwrap();
            let graph: bil_mir::BilMirGraph = serde_json::from_str(&json).unwrap();
            let artifact = bil_sdk::Bil::issue(graph, bil_ink::CapabilityCode("demo.receipt".to_string())).unwrap();
            
            let receipt_json = serde_json::to_string_pretty(&artifact.receipt).unwrap();
            fs::write(&out, &receipt_json).unwrap();
            println!("Issued receipt to {}", out);
        }
        Commands::Verify { receipt, out, pretty } => {
            let json = fs::read_to_string(&receipt).unwrap();
            let receipt_obj: bil_ink::InkReceipt = serde_json::from_str(&json).unwrap();
            let report = bil_sdk::Bil::verify(&receipt_obj).unwrap();
            
            if let Some(out_path) = out {
                let report_json = serde_json::to_string_pretty(&report).unwrap();
                fs::write(out_path, report_json).unwrap();
                println!("Wrote verification report to {}", out_path);
            } else if *pretty {
                println!("Verification Report:");
                println!("  Status: {:?}", report.status);
                println!("  Receipt ID: {:?}", report.receipt_id);
                println!("  Profile: {:?}", report.profile);
                println!("  Checks:");
                for check in &report.checks {
                    println!("    - {:?}: {:?}", check.kind, check.status);
                }
                if !report.findings.is_empty() {
                    println!("  Findings:");
                    for finding in &report.findings {
                        println!("    - [{:?}] {}", finding.priority, finding.message);
                    }
                }
            } else {
                let report_json = serde_json::to_string_pretty(&report).unwrap();
                println!("{}", report_json);
            }
        }
        Commands::Explain { receipt, report, out } => {
            let json = fs::read_to_string(&receipt).unwrap();
            let receipt_obj: bil_ink::InkReceipt = serde_json::from_str(&json).unwrap();
            
            let report_obj = if let Some(report_path) = report {
                let report_json = fs::read_to_string(report_path).unwrap();
                serde_json::from_str(&report_json).unwrap()
            } else {
                bil_sdk::Bil::verify(&receipt_obj).unwrap()
            };

            let explanation = bil_sdk::Bil::explain(&report_obj);
            
            if let Some(out_path) = out {
                fs::write(out_path, &explanation.markdown).unwrap();
                println!("Wrote explanation to {}", out_path);
            } else {
                println!("{}", explanation.markdown);
            }
        }
        Commands::Conformance { group } => {
            if let Err(e) = bil_conformance::run_conformance_group(group) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
