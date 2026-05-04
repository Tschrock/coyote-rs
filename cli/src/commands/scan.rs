use std::{error::Error, time::Duration};

use btleplug::{
    api::{Central, CentralEvent, Manager as _, ScanFilter},
    platform::Manager,
};
use futures_util::StreamExt;
use log::{error, info, trace};
use tokio::time;
use tokio_util::sync::CancellationToken;

use crate::{cli::Scan, commands::util::Plural};

pub async fn scan_devices(options: &Scan) -> Result<(), Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await?;

    if adapters.is_empty() {
        error!("No bluetooth adapters found");
        std::process::exit(1);
    }

    trace!(
        "Found {} adapter{}",
        adapters.len(),
        adapters.len().plural("", "s")
    );

    let index = match options.adapter {
        Some(index) => index,
        None => {
            trace!("No adapter index provided, using index 0");
            0
        }
    };

    let adapter = match adapters.get(index) {
        Some(adapter) => adapter,
        None => {
            error!("Adapter index out of range");
            std::process::exit(1);
        }
    };

    info!("Using adapter: {:?}", adapter.adapter_info().await?);
    let mut events = adapter.events().await?;

    info!("Starting scan for devices");
    adapter.start_scan(ScanFilter::default()).await?;

    let cancel_token = CancellationToken::new();

    // start a timer to stop scanning after the timeout
    let adapter2 = adapter.clone();
    let cancel_token2 = cancel_token.clone();
    let timeout = Duration::from_secs(options.timeout);

    let scan_timeout_task = tokio::spawn(async move {
        time::sleep(timeout).await;
        adapter2.stop_scan().await.unwrap();
        cancel_token2.cancel();
    });

    let event_processing_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                biased;
                _ = cancel_token.cancelled() => {
                    break;
                },
                Some(event) = events.next() => {
                    match event {
                        CentralEvent::DeviceDiscovered(id) => {
                            println!("DeviceDiscovered: {:?}", id);
                        }
                        _ => {}
                    }
                },
                else => {
                    break;
                }
            }
        }
    });

    scan_timeout_task.await?;
    event_processing_task.await?;

    Ok(())
}
