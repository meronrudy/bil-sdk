//! AiSSURANCE CLI entry point for alpha risk and platform demos.

use aissurance_cli::{run_demo, run_platform_demo};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aissurance")]
#[command(about = "AiSSURANCE alpha platform CLI")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run deterministic risk-layer demo data through the batch facade.
    Demo {
        #[arg(short, long)]
        config: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
    /// Run the alpha platform demo across planner, safety, and control plane.
    PlatformDemo {
        #[arg(short, long)]
        config: Option<PathBuf>,
        #[arg(long)]
        json: bool,
        #[arg(long, default_value = ".aissurance-alpha")]
        data_dir: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Demo { config, json } => handle_demo(config, json)?,
        Commands::PlatformDemo {
            config,
            json,
            data_dir,
        } => handle_platform_demo(config, json, data_dir)?,
    }

    Ok(())
}

fn handle_demo(config_path: Option<PathBuf>, json: bool) -> anyhow::Result<()> {
    let result = run_demo(config_path)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&result.report)?);
    } else {
        println!("AiSSURANCE risk-layer demo complete");
        println!("Frames ingested: {}", result.report.frames_ingested);
        println!("Frames rejected: {}", result.report.frames_rejected);
        println!("Risk events: {}", result.report.risk_events.len());
        println!("Feature bundles: {}", result.report.feature_bundles.len());
        println!("Premiums: {}", result.report.premiums.len());
        println!(
            "Fallback model: {}",
            result.report.model.used_fallback_model
        );
    }

    Ok(())
}

fn handle_platform_demo(
    config_path: Option<PathBuf>,
    json: bool,
    data_dir: PathBuf,
) -> anyhow::Result<()> {
    let result = run_platform_demo(config_path, data_dir)?;

    if json {
        println!(
            "{}",
            serde_json::json!({
                "planner": result.planner,
                "safety": result.safety,
                "job": result.job,
                "risk_report": result.risk_report,
            })
        );
    } else {
        println!("AiSSURANCE alpha platform demo complete");
        println!("Planner status: {:?}", result.planner.status);
        println!("Safety decisions: {}", result.safety.decisions.len());
        println!(
            "Safety max latency: {}us",
            result.safety.max_latency.as_micros()
        );
        println!("Control-plane job: {}", result.job.job_id);
        println!("Persisted report: {:?}", result.job.artifacts.report_path);
        println!("Risk premiums: {}", result.risk_report.premiums.len());
    }

    Ok(())
}
