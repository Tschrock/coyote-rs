mod subcommands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "toolkit", about = "Coyote BLE toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Man-in-the-middle a BLE device
    Mitm {
        #[command(subcommand)]
        target: Target,
    },
    /// Scan commands from a BLE device
    Scan {
        #[command(subcommand)]
        target: Target,
    },
}

#[derive(Subcommand)]
enum Target {
    Coyote,
    Pawprint,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Mitm { target } => match target {
            Target::Coyote => subcommands::mitm::mitm_coyote().await,
            Target::Pawprint => subcommands::mitm::mitm_pawprint().await,
        },
        Command::Scan { target } => match target {
            Target::Coyote => subcommands::scan::scan_coyote().await,
            Target::Pawprint => subcommands::scan::scan_pawprint().await,
        },
    }
}
