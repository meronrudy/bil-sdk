use bil_mir::BilMirGraph;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

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
        #[arg(short, long, default_value = "human-override")]
        profile: String,
    },
    /// Check environment capabilities
    Doctor,
    /// Generate a synthetic workflow
    Mock {
        /// Generator family or profile name. Use `generic` for public BIL mocks.
        target: String,
        #[arg(long, default_value = "human-override")]
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
        /// Input MIR graph path.
        #[arg(long)]
        input: Option<String>,
        /// Output receipt JSON path.
        #[arg(long)]
        out: Option<String>,
        /// Receipt capability code.
        #[arg(long, default_value = "assurance-receipt")]
        capability: String,
        /// Backward-compatible positional capability or input path.
        positional_1: Option<String>,
        /// Backward-compatible positional MIR path.
        positional_2: Option<String>,
    },
    /// Verify a receipt
    Verify {
        #[arg(long)]
        receipt: Option<String>,
        #[arg(long)]
        out: Option<String>,
        #[arg(long)]
        pretty: bool,
        /// Backward-compatible positional receipt path.
        receipt_file: Option<String>,
    },
    /// Explain verification findings
    Explain {
        #[arg(long)]
        receipt: Option<String>,
        #[arg(long)]
        report: Option<String>,
        #[arg(long)]
        out: Option<String>,
        /// Backward-compatible positional receipt path.
        receipt_file: Option<String>,
    },
    /// Run conformance tests
    Conformance { group: String },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Demo { profile } => {
            let Some(demo_profile) = bil_sdk::DemoProfile::from_name(profile) else {
                eprintln!("Unknown profile: {}", profile);
                std::process::exit(2);
            };

            let demo = bil_sdk::Bil::demo(demo_profile).unwrap();
            let receipt = demo.receipt().unwrap();
            let memo = demo.memo().unwrap();

            fs::create_dir_all("artifacts/demo").unwrap();

            let receipt_json = serde_json::to_string_pretty(&receipt).unwrap();
            write_text("artifacts/demo/ink_receipt.v1.json", &receipt_json).unwrap();

            memo.write_markdown("artifacts/demo/assurance_memo.md")
                .unwrap();

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
        Commands::Mock {
            target,
            profile,
            seed,
            out,
        } => {
            let Some(mock_profile) = resolve_mock_profile(target, profile) else {
                eprintln!("Unknown mock profile: {}", profile);
                std::process::exit(2);
            };
            let graph = bil_sdk::Bil::mock(mock_profile)
                .with_seed(seed.unwrap_or(0))
                .build()
                .unwrap();
            let json = serde_json::to_string_pretty(&graph).unwrap();
            write_or_print(out.as_deref(), &json, "mock workflow").unwrap();
        }
        Commands::Build { workflow_file, out } => {
            let json = fs::read_to_string(workflow_file).unwrap();
            let graph: BilMirGraph = serde_json::from_str(&json).unwrap();
            let out_json = serde_json::to_string_pretty(&graph).unwrap();
            write_or_print(out.as_deref(), &out_json, "built MIR").unwrap();
        }
        Commands::Issue {
            input,
            out,
            capability,
            positional_1,
            positional_2,
        } => {
            let (input_path, capability_code) =
                resolve_issue_args(input, capability, positional_1, positional_2);
            let json = fs::read_to_string(&input_path).unwrap();
            let graph = parse_issue_input(&json).unwrap();
            let artifact =
                bil_sdk::Bil::issue(graph, bil_ink::CapabilityCode(capability_code)).unwrap();

            let receipt_json = serde_json::to_string_pretty(&artifact.receipt).unwrap();
            if let Some(out_path) = out {
                write_text(out_path, &receipt_json).unwrap();
                println!("Issued receipt to {}", out_path);
            } else {
                println!("{}", receipt_json);
            }
        }
        Commands::Verify {
            receipt,
            out,
            pretty,
            receipt_file,
        } => {
            let receipt_path = receipt
                .as_deref()
                .or(receipt_file.as_deref())
                .expect("receipt path is required");
            let json = fs::read_to_string(receipt_path).unwrap();
            let receipt_obj: bil_ink::InkReceipt = serde_json::from_str(&json).unwrap();
            let report = bil_sdk::Bil::verify(&receipt_obj).unwrap();

            if let Some(out_path) = out {
                let report_json = serde_json::to_string_pretty(&report).unwrap();
                write_text(out_path, &report_json).unwrap();
                println!("Wrote verification report to {}", out_path);
            } else if *pretty {
                print_pretty_report(&report);
            } else {
                let report_json = serde_json::to_string_pretty(&report).unwrap();
                println!("{}", report_json);
            }
        }
        Commands::Explain {
            receipt,
            report,
            out,
            receipt_file,
        } => {
            let receipt_path = receipt
                .as_deref()
                .or(receipt_file.as_deref())
                .expect("receipt path is required");
            let json = fs::read_to_string(receipt_path).unwrap();
            let receipt_obj: bil_ink::InkReceipt = serde_json::from_str(&json).unwrap();

            let report_obj = if let Some(report_path) = report {
                let report_json = fs::read_to_string(report_path).unwrap();
                serde_json::from_str(&report_json).unwrap()
            } else {
                bil_sdk::Bil::verify(&receipt_obj).unwrap()
            };

            let explanation = bil_sdk::Bil::explain(&report_obj);

            if let Some(out_path) = out {
                write_text(out_path, &explanation.markdown).unwrap();
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

fn resolve_mock_profile(target: &str, profile: &str) -> Option<bil_sdk::SyntheticProfile> {
    if normalize_name(target) == "generic" {
        bil_sdk::SyntheticProfile::from_name(profile)
    } else {
        bil_sdk::SyntheticProfile::from_name(target)
    }
}

fn resolve_issue_args(
    input: &Option<String>,
    capability: &str,
    positional_1: &Option<String>,
    positional_2: &Option<String>,
) -> (String, String) {
    if let Some(input_path) = input {
        return (input_path.clone(), capability.to_string());
    }

    match (positional_1, positional_2) {
        (Some(input_path), None) => (input_path.clone(), capability.to_string()),
        (Some(capability_code), Some(input_path)) => (input_path.clone(), capability_code.clone()),
        _ => panic!("issue requires --input <mir.json> or positional MIR input"),
    }
}

fn parse_issue_input(json: &str) -> Result<BilMirGraph, serde_json::Error> {
    serde_json::from_str::<BilMirGraph>(json)
}

fn normalize_name(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('-', "_")
}

fn write_or_print(out: Option<&str>, contents: &str, label: &str) -> std::io::Result<()> {
    if let Some(out_path) = out {
        write_text(out_path, contents)?;
        println!("Wrote {} to {}", label, out_path);
    } else {
        println!("{}", contents);
    }
    Ok(())
}

fn write_text(path: impl AsRef<Path>, contents: &str) -> std::io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    if contents.ends_with('\n') {
        fs::write(path, contents)
    } else {
        fs::write(path, format!("{}\n", contents))
    }
}

fn print_pretty_report(report: &bil_verify::VerificationReport) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_mock_graph_is_issueable() {
        let graph = bil_sdk::Bil::mock(bil_sdk::SyntheticProfile::HumanOverride)
            .with_seed(2026)
            .build()
            .unwrap();

        assert_eq!(graph.profile.0, "human_override");
        assert!(!graph.events.is_empty());
        assert!(!graph.evidence.is_empty());
        assert!(!graph.authorities.is_empty());
        assert!(!graph.policies.is_empty());

        let artifact = bil_sdk::Bil::issue(
            graph,
            bil_ink::CapabilityCode("assurance-receipt".to_string()),
        )
        .unwrap();
        let report = bil_sdk::Bil::verify(&artifact.receipt).unwrap();

        assert_ne!(report.status, bil_core::BilStatus::Fail);
    }

    #[test]
    fn mock_profile_resolver_accepts_generic_form() {
        let profile = resolve_mock_profile("generic", "human-override").unwrap();
        assert_eq!(profile, bil_sdk::SyntheticProfile::HumanOverride);
    }

    #[test]
    fn issue_input_parser_accepts_mir_graph() {
        let graph = bil_sdk::generic_human_override_graph(2026);
        let json = serde_json::to_string(&graph).unwrap();
        let parsed = parse_issue_input(&json).unwrap();

        assert_eq!(parsed.graph_id.0, graph.graph_id.0);
        assert_eq!(parsed.profile.0, "human_override");
    }
}
