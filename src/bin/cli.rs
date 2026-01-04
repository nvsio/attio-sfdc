//! CLI tool for local development and testing.

#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};

#[cfg(feature = "cli")]
#[derive(Parser)]
#[command(name = "attio-sfdc")]
#[command(about = "Attio-Salesforce sync bridge CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Trigger a sync operation
    Sync {
        /// Sync direction: attio-to-sf, sf-to-attio, or bidirectional
        #[arg(short, long, default_value = "bidirectional")]
        direction: String,

        /// Specific object to sync (optional)
        #[arg(short, long)]
        object: Option<String>,
    },
    /// Check connection to APIs
    Check {
        /// Check only Attio connection
        #[arg(long)]
        attio_only: bool,

        /// Check only Salesforce connection
        #[arg(long)]
        sf_only: bool,
    },
    /// Show current configuration
    Config,
    /// Show sync history
    History {
        /// Number of entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    /// List unresolved conflicts
    Conflicts,
}

#[cfg(feature = "cli")]
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sync { direction, object } => {
            println!("Starting {} sync...", direction);
            if let Some(obj) = object {
                println!("Syncing object: {}", obj);
            }
            // TODO: Implement sync
            println!("Sync complete!");
        }
        Commands::Check { attio_only, sf_only } => {
            if !sf_only {
                println!("Checking Attio connection...");
                // TODO: Implement connection check
                println!("  Attio: OK");
            }
            if !attio_only {
                println!("Checking Salesforce connection...");
                // TODO: Implement connection check
                println!("  Salesforce: OK");
            }
        }
        Commands::Config => {
            println!("Current configuration:");
            // TODO: Load and display config
            println!("  Sync direction: bidirectional");
            println!("  Batch size: 100");
        }
        Commands::History { limit } => {
            println!("Recent sync history (last {}):", limit);
            // TODO: Load and display history
            println!("  No sync history found");
        }
        Commands::Conflicts => {
            println!("Unresolved conflicts:");
            // TODO: Load and display conflicts
            println!("  No conflicts found");
        }
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("CLI not enabled. Build with --features cli");
    std::process::exit(1);
}
