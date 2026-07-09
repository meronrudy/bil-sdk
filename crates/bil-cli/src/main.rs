use anyhow::{anyhow, Context, Result};
use bil_mir::BilMirGraph;
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[cfg(feature = "aissurance-local")]
use bil_aissurance_bridge::{run_platform_demo_and_issue, AissurancePlatformDemoOptions};

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
    #[cfg(feature = "aissurance-local")]
    /// Run local AiSSURANCE integration flows
    Aissurance {
        #[command(subcommand)]
        command: AissuranceCommands,
    },
}

#[cfg(feature = "aissurance-local")]
#[derive(Subcommand)]
enum AissuranceCommands {
    /// Run the AiSSURANCE full platform demo and issue BIL receipts
    PlatformDemo {
        #[arg(long)]
        config: Option<String>,
        #[arg(long, default_value = ".aissurance-alpha")]
        data_dir: String,
        #[arg(long, default_value = "artifacts/aissurance")]
        out_dir: String,
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Demo { profile } => {
            let demo_profile = bil_sdk::DemoProfile::from_name(profile)
                .ok_or_else(|| anyhow!("unknown profile: {}", profile))?;

            let demo = bil_sdk::Bil::demo(demo_profile)?;
            let receipt = demo.receipt()?;
            let memo = demo.memo()?;

            fs::create_dir_all("artifacts/demo")
                .context("failed to create artifacts/demo directory")?;

            let receipt_json =
                serde_json::to_string_pretty(&receipt).context("failed to encode receipt JSON")?;
            write_text("artifacts/demo/ink_receipt.v1.json", &receipt_json)?;

            memo.write_markdown("artifacts/demo/assurance_memo.md")
                .context("failed to write assurance memo")?;

            println!("Demo completed successfully.");
            println!("Artifacts written to artifacts/demo/");
        }
        Commands::Doctor => {
            let report = bil_sdk::Bil::doctor()?;
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
            let mock_profile = resolve_mock_profile(target, profile)
                .ok_or_else(|| anyhow!("unknown mock profile: {}", profile))?;
            let graph = bil_sdk::Bil::mock(mock_profile)
                .with_seed(seed.unwrap_or(0))
                .build()?;
            let json =
                serde_json::to_string_pretty(&graph).context("failed to encode mock MIR JSON")?;
            write_or_print(out.as_deref(), &json, "mock workflow")?;
        }
        Commands::Build { workflow_file, out } => {
            let json = read_text_file(workflow_file)?;
            let graph: BilMirGraph = serde_json::from_str(&json)
                .with_context(|| format!("failed to parse MIR graph from {}", workflow_file))?;
            let out_json =
                serde_json::to_string_pretty(&graph).context("failed to encode MIR JSON")?;
            write_or_print(out.as_deref(), &out_json, "built MIR")?;
        }
        Commands::Issue {
            input,
            out,
            capability,
            positional_1,
            positional_2,
        } => {
            let (input_path, capability_code) =
                resolve_issue_args(input, capability, positional_1, positional_2)?;
            let json = read_text_file(&input_path)?;
            let graph = parse_issue_input(&json)
                .with_context(|| format!("failed to parse MIR graph from {}", input_path))?;
            let artifact = bil_sdk::Bil::issue(graph, bil_ink::CapabilityCode(capability_code))?;

            let receipt_json = serde_json::to_string_pretty(&artifact.receipt)
                .context("failed to encode receipt JSON")?;
            if let Some(out_path) = out {
                write_text(out_path, &receipt_json)?;
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
            let receipt_path = resolve_receipt_path(receipt, receipt_file)?;
            let json = read_text_file(receipt_path)?;
            let receipt_obj: bil_ink::InkReceipt = serde_json::from_str(&json)
                .with_context(|| format!("failed to parse receipt from {}", receipt_path))?;
            let report = bil_sdk::Bil::verify(&receipt_obj)?;

            if let Some(out_path) = out {
                let report_json = serde_json::to_string_pretty(&report)
                    .context("failed to encode verification report JSON")?;
                write_text(out_path, &report_json)?;
                println!("Wrote verification report to {}", out_path);
            } else if *pretty {
                print_pretty_report(&report);
            } else {
                let report_json = serde_json::to_string_pretty(&report)
                    .context("failed to encode verification report JSON")?;
                println!("{}", report_json);
            }
        }
        Commands::Explain {
            receipt,
            report,
            out,
            receipt_file,
        } => {
            let receipt_path = resolve_receipt_path(receipt, receipt_file)?;
            let json = read_text_file(receipt_path)?;
            let receipt_obj: bil_ink::InkReceipt = serde_json::from_str(&json)
                .with_context(|| format!("failed to parse receipt from {}", receipt_path))?;

            let report_obj = if let Some(report_path) = report {
                let report_json = read_text_file(report_path)?;
                serde_json::from_str(&report_json)
                    .with_context(|| format!("failed to parse report from {}", report_path))?
            } else {
                bil_sdk::Bil::verify(&receipt_obj)?
            };

            let explanation = bil_sdk::Bil::explain(&report_obj);

            if let Some(out_path) = out {
                write_text(out_path, &explanation.markdown)?;
                println!("Wrote explanation to {}", out_path);
            } else {
                println!("{}", explanation.markdown);
            }
        }
        Commands::Conformance { group } => {
            bil_conformance::run_conformance_group(group).map_err(|e| anyhow!(e))?;
        }
        #[cfg(feature = "aissurance-local")]
        Commands::Aissurance { command } => match command {
            AissuranceCommands::PlatformDemo {
                config,
                data_dir,
                out_dir,
                json,
            } => {
                let manifest = run_platform_demo_and_issue(AissurancePlatformDemoOptions {
                    config_path: config.as_ref().map(Into::into),
                    data_dir: data_dir.into(),
                    out_dir: out_dir.into(),
                })?;

                if *json {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&manifest)
                            .context("failed to encode AiSSURANCE manifest JSON")?
                    );
                } else {
                    println!("AiSSURANCE local platform demo completed.");
                    println!("  Run ID: {}", manifest.run_id);
                    println!("  Job ID: {}", manifest.job_id);
                    println!("  Planner receipt: {}", manifest.planner_receipt.receipt_id);
                    println!("  Safety receipt: {}", manifest.safety_receipt.receipt_id);
                    println!("  Risk receipt: {}", manifest.risk_receipt.receipt_id);
                    println!("  Aggregate receipt: {}", manifest.run_receipt.receipt_id);
                    println!("  Manifest: {}", manifest.manifest_path.display());
                }
            }
        },
    }

    Ok(())
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
) -> Result<(String, String)> {
    if let Some(input_path) = input {
        return Ok((input_path.clone(), capability.to_string()));
    }

    match (positional_1, positional_2) {
        (Some(input_path), None) => Ok((input_path.clone(), capability.to_string())),
        (Some(capability_code), Some(input_path)) => {
            Ok((input_path.clone(), capability_code.clone()))
        }
        _ => Err(anyhow!(
            "issue requires --input <mir.json> or positional MIR input"
        )),
    }
}

fn resolve_receipt_path<'a>(
    receipt: &'a Option<String>,
    receipt_file: &'a Option<String>,
) -> Result<&'a str> {
    receipt
        .as_deref()
        .or(receipt_file.as_deref())
        .ok_or_else(|| anyhow!("receipt path is required"))
}

fn parse_issue_input(json: &str) -> Result<BilMirGraph, serde_json::Error> {
    serde_json::from_str::<BilMirGraph>(json)
}

fn normalize_name(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace('-', "_")
}

fn read_text_file(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
}

fn write_or_print(out: Option<&str>, contents: &str, label: &str) -> Result<()> {
    if let Some(out_path) = out {
        write_text(out_path, contents)?;
        println!("Wrote {} to {}", label, out_path);
    } else {
        println!("{}", contents);
    }
    Ok(())
}

fn write_text(path: impl AsRef<Path>, contents: &str) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
    }
    if contents.ends_with('\n') {
        fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))
    } else {
        fs::write(path, format!("{}\n", contents))
            .with_context(|| format!("failed to write {}", path.display()))
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

    #[test]
    fn issue_args_require_input() {
        let err = resolve_issue_args(&None, "assurance-receipt", &None, &None).unwrap_err();
        assert!(err.to_string().contains("--input <mir.json>"));
    }

    #[test]
    fn receipt_path_is_required() {
        let err = resolve_receipt_path(&None, &None).unwrap_err();
        assert_eq!(err.to_string(), "receipt path is required");
    }

    #[test]
    fn read_text_file_reports_bad_input_path() {
        let missing =
            std::env::temp_dir().join(format!("bil-cli-missing-input-{}.json", std::process::id()));
        let err = read_text_file(&missing).unwrap_err();
        assert!(err.to_string().contains("failed to read"));
    }

    #[test]
    fn issue_input_parser_rejects_malformed_mir_json() {
        assert!(parse_issue_input("{").is_err());
    }
}
