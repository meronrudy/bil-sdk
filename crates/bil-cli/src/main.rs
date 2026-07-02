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
        capability: String,
        mir_file: String,
    },
    /// Verify a receipt
    Verify {
        receipt_file: String,
        #[arg(long)]
        pretty: bool,
    },
    /// Explain verification findings
    Explain {
        receipt_file: String,
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
            println!("Running demo for profile: {}", profile);
        }
        Commands::Doctor => {
            println!("Checking environment...");
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
        Commands::Issue { capability, mir_file } => {
            println!("Issuing {} from {}", capability, mir_file);
        }
        Commands::Verify { receipt_file, pretty } => {
            println!("Verifying receipt: {} (pretty: {})", receipt_file, pretty);
        }
        Commands::Explain { receipt_file } => {
            println!("Explaining receipt: {}", receipt_file);
        }
        Commands::Conformance { group } => {
            println!("Running conformance group: {}", group);
        }
    }
}
