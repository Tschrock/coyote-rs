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
    /// Proxy a BLE device
    Proxy {
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
        Command::Proxy { target } => match target {
            Target::Coyote => subcommands::proxy::proxy_coyote().await,
            Target::Pawprint => subcommands::proxy::proxy_pawprint().await,
        },
        Command::Scan { target } => match target {
            Target::Coyote => subcommands::scan::scan_coyote().await,
            Target::Pawprint => subcommands::scan::scan_pawprint().await,
        },
    }
}
