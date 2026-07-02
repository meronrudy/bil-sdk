use clap::{Parser, Subcommand};

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
            println!("Generating mock for profile: {}, seed: {:?}, out: {:?}", profile, seed, out);
        }
        Commands::Build { workflow_file, out } => {
            println!("Building MIR from: {}, out: {:?}", workflow_file, out);
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
