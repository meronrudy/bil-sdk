use clap::{Parser, Subcommand};
use construction::ConstructionPack;
use mechassure_core::DomainPack;
use mechassure_underwriting::UnderwritingFile;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "mechassure")]
#[command(about = "AIssurance Evidence SDK CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a demo underwriting file for a specific domain
    Demo {
        /// The domain to generate a demo for (e.g., construction)
        domain: String,
    },
    /// Initialize a new AI system project
    Init {
        #[arg(long)]
        domain: String,
    },
    /// Reduce raw logs into risk statistics
    Reduce {
        logs_dir: PathBuf,
        #[arg(long)]
        domain: String,
        #[arg(long)]
        out: PathBuf,
    },
    /// Export an underwriting file to a specific format
    Export {
        underwriting_file_dir: PathBuf,
        #[arg(long)]
        format: String,
    },
    /// Validate an underwriting file
    Validate {
        underwriting_file_dir: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Demo { domain } => {
            if domain == "construction" {
                println!("Generating demo underwriting file for construction domain...");
                let pack = ConstructionPack;
                
                let mut file = UnderwritingFile::default();
                file.domain = pack.id().to_string();
                file.insured_system = "Demo Autonomous Excavator v2.1".to_string();
                file.period = "2024-01-01 to 2024-12-31".to_string();
                
                file.exposure = json!({
                    "autonomous_operation_hours": 1500,
                    "human_machine_interaction_hours": 300,
                    "critical_task_count": 450
                });
                
                file.risk_statistics = json!({
                    "human_proximity_events": 12,
                    "exclusion_zone_violations": 2,
                    "emergency_stops": 5,
                    "telemetry_completeness_score": 0.98
                });
                
                let json_output = serde_json::to_string_pretty(&file).unwrap();
                println!("{}", json_output);
                
                fs::write("demo_underwriting_file.json", json_output).unwrap();
                println!("Saved to demo_underwriting_file.json");
            } else {
                println!("Demo for domain '{}' is not yet implemented.", domain);
            }
        }
        Commands::Init { domain } => {
            println!("Initializing new AI system project for domain: {}", domain);
        }
        Commands::Reduce { logs_dir, domain, out } => {
            println!("Reducing logs from {:?} for domain {} to {:?}", logs_dir, domain, out);
        }
        Commands::Export { underwriting_file_dir, format } => {
            println!("Exporting underwriting file from {:?} to format {}", underwriting_file_dir, format);
        }
        Commands::Validate { underwriting_file_dir } => {
            println!("Validating underwriting file at {:?}", underwriting_file_dir);
        }
    }
}
