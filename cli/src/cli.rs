use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lists all useable Bluetooth adapters
    List,
    Scan(Scan),
    Battery(Battery),
}


#[derive(Parser)]
/// Scans for Coyote devices
pub struct Scan {
    /// The time to spend scanning for devices, in seconds
    #[clap(short, long, default_value = "6")]
    pub timeout: u64,
    /// The adapter to use for scanning. To get an adapter index, use the `list` command. If not provided, the first adapter found will be used.
    #[clap(short, long)]
    pub adapter: Option<usize>,
}

#[derive(Parser)]
/// Gets the battery level of a device
pub struct Battery {
    /// The maximum time to spend looking for the device, in seconds
    #[clap(short, long, default_value = "6")]
    pub timeout: u64,
    /// The adapter to use for scanning. To get an adapter index, use the `list` command. If not provided, the first adapter found will be used.
    #[clap(short, long)]
    pub adapter: Option<usize>,
    /// The device to get the battery level of. To get a device index, use the `scan` command. If not provided, the first device found will be used.
    #[clap(short, long)]
    pub device: Option<usize>,
}
