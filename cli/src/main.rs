// use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter, WriteType};
// use btleplug::platform::{Adapter, Manager, Peripheral};
use clap::Parser;
use cli::Cli;
use commands::battery::get_battery;
use commands::list::list_adapters;
use commands::scan::scan_devices;
use log::error;
// use coyote_connect::ble::v3::raw::COMAMND_WRITE_SERVICE_UUID;
// use log::{error, info, trace};
// use rand::{thread_rng, Rng};
use std::error::Error;
// use std::time::Duration;
// use tokio::time;

mod cli;
mod commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Some(cmd) => match cmd {
            cli::Commands::List => list_adapters().await?,
            cli::Commands::Scan(options) => scan_devices(options).await?,
            cli::Commands::Battery(options) => get_battery(options).await?,
        },
        None => {
            error!("No command provided");
            std::process::exit(1);
        }
    };

    return Ok(());

    // trace!("Init bluetooth manager");
    // let manager = Manager::new().await.unwrap();

    // trace!("Get adapters");
    // let adapters = manager.adapters().await?;

    // info!("Found {} bluetooth adapters", adapters.len());
    // let central = adapters.into_iter().nth(0).unwrap();
    // trace!("Using adapter: {:?}", central.adapter_info().await?);

    // info!("Starting scan for devices");

    // // Get the events stream
    // let mut events = central.events().await?;

    // // Start scanning
    // central
    //     .start_scan(ScanFilter {
    //         services: vec![COMAMND_WRITE_SERVICE_UUID],
    //     })
    //     .await?;

    // // TODO: use central.events() to get a stream of new devices
    // trace!("Waiting 2 seconds to find devices");
    // time::sleep(Duration::from_secs(2)).await;

    // // find the device we're interested in
    // let light = find_light(&central).await.unwrap();

    // // connect to the device
    // light.connect().await?;

    // // discover services and characteristics
    // light.discover_services().await?;

    // // find the characteristic we want
    // let chars = light.characteristics();
    // let cmd_char = chars
    //     .iter()
    //     .find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID)
    //     .unwrap();

    // // dance party
    // let mut rng = thread_rng();
    // for _ in 0..20 {
    //     let color_cmd = vec![0x56, rng.gen(), rng.gen(), rng.gen(), 0x00, 0xF0, 0xAA];
    //     light
    //         .write(&cmd_char, &color_cmd, WriteType::WithoutResponse)
    //         .await?;
    //     time::sleep(Duration::from_millis(200)).await;
    // }
    // Ok(())
}

// async fn find_light(central: &Adapter) -> Option<Peripheral> {
//     for p in central.peripherals().await.unwrap() {
//         if p.properties()
//             .await
//             .unwrap()
//             .unwrap()
//             .local_name
//             .iter()
//             .any(|name| name.contains("LEDBlue"))
//         {
//             return Some(p);
//         }
//     }
//     None
// }
