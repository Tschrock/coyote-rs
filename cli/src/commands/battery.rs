use std::{pin::Pin, time::Duration};

use crate::cli::Battery;
use btleplug::{
    api::{Central, CentralEvent, Manager as _, Peripheral, ScanFilter},
    platform::{Manager, PeripheralId},
};
use coyote_connect_ble::v3::PULSE_HOST_DEFAULT_NAME;
use futures_util::{Stream, StreamExt};
use log::{info, trace, warn};
use thiserror::Error;
use tokio::{sync::mpsc, time};
use tokio_util::sync::CancellationToken;

#[derive(Error, Debug)]
pub enum GetBatteryError {
    #[error("Error initializing bluetooth manager: {0}")]
    ManagerError(btleplug::Error),
    #[error("Error getting adapters: {0}")]
    AdapterError(btleplug::Error),
    #[error("No adapters found")]
    NoAdaptersFound,
    #[error("Adapter {0} not found")]
    AdapterNotFound(usize),
    #[error("Error getting event stream: {0}")]
    EventStreamError(btleplug::Error),
    #[error("Error starting scan: {0}")]
    ScanError(btleplug::Error),
    #[error("Error reading battery level: {0}")]
    ReadBatteryError(String),
}

pub async fn get_battery(options: &Battery) -> Result<(), GetBatteryError> {
    // Get the bluetooth manager
    let manager = match Manager::new().await {
        Ok(manager) => manager,
        Err(e) => return Err(GetBatteryError::ManagerError(e)),
    };

    // Get the list of adapters
    let adapters = match manager.adapters().await {
        Ok(adapters) => adapters,
        Err(e) => return Err(GetBatteryError::AdapterError(e)),
    };

    // Check if there are any adapters
    if adapters.is_empty() {
        return Err(GetBatteryError::NoAdaptersFound);
    }

    // Get the desired adapter
    let adapter_index = options.adapter.unwrap_or(0);
    let adapter = match adapters.get(adapter_index) {
        Some(adapter) => adapter,
        None => return Err(GetBatteryError::AdapterNotFound(adapter_index)),
    };

    // Get the events stream
    let events = match adapter.events().await {
        Ok(events) => events,
        Err(e) => return Err(GetBatteryError::EventStreamError(e)),
    };

    // Build the scan filter
    let mut scan_filter = ScanFilter::default();
    scan_filter.services.append(&mut vec![
        coyote_connect_ble::v3::attributes::SERVICE_DEVICE_INFORMATION,
        coyote_connect_ble::v3::attributes::SERVICE_PULSE_HOST,
    ]);

    // Start scanning
    match adapter.start_scan(scan_filter).await {
        Ok(_) => (),
        Err(e) => return Err(GetBatteryError::ScanError(e)),
    };

    println!("Scanning for devices...");
    let (devices_tx, devices_rx) = mpsc::channel(16);
    let cancel_token = CancellationToken::new();
    let scan_timeout_task = tokio::spawn(scan_timeout_loop(
        adapter.clone(),
        Duration::from_secs(options.timeout),
        cancel_token.clone(),
    ));
    let event_processing_task = tokio::spawn(process_events_loop(
        adapter.clone(),
        events,
        cancel_token,
        devices_tx,
    ));
    let device_reading_task = tokio::spawn(read_battery_loop(adapter.clone(), devices_rx));

    tokio::try_join!(
        scan_timeout_task,
        event_processing_task,
        device_reading_task
    )
    .map_err(|e| GetBatteryError::ReadBatteryError(e.to_string()))?;

    Ok(())
}

async fn process_events_loop<A: Central>(
    adapter: A,
    mut events: Pin<Box<dyn Stream<Item = CentralEvent> + Send>>,
    cancel_token: CancellationToken,
    devices_tx: mpsc::Sender<PeripheralId>,
) {
    loop {
        tokio::select! {
            biased;
            _ = cancel_token.cancelled() => {
                break;
            },
            Some(event) = events.next() => {
                match event {
                    CentralEvent::DeviceDiscovered(id) => {
                        handle_device_discovered(&adapter, id, &devices_tx).await;
                    }
                    _ => {}
                }
            },
            else => {
                break;
            }
        }
    }
    drop(devices_tx); // idk if needed
}

async fn handle_device_discovered<A: Central>(
    adapter: &A,
    id: PeripheralId,
    devices_tx: &mpsc::Sender<PeripheralId>,
) {
    let device = adapter.peripheral(&id).await.unwrap();
    let device_info = device.properties().await.unwrap().unwrap();
    match device_info.local_name {
        Some(name) => {
            if name == PULSE_HOST_DEFAULT_NAME {
                info!("Found Coyote device {:?} ({})", id, name);
                devices_tx.send(id).await.unwrap();
            } else {
                trace!("Device {:?} ({}) is not a known Coyote device", id, name);
            }
        }
        None => {
            warn!("Device {:?} has no local name", id);
        }
    }
}

async fn scan_timeout_loop<A: Central>(
    adapter: A,
    timeout: std::time::Duration,
    cancel_token: CancellationToken,
) {
    time::sleep(timeout).await;
    adapter.stop_scan().await.unwrap();
    cancel_token.cancel();
}

async fn read_battery_loop<A: Central>(adapter: A, mut devices_rx: mpsc::Receiver<PeripheralId>) {
    while let Some(id) = devices_rx.recv().await {
        let device = adapter.peripheral(&id).await.unwrap();
        device.discover_services().await.unwrap();
        let chars = device.characteristics();
        let battery_char = chars
            .iter()
            .find(|c| {
                c.service_uuid == coyote_connect_ble::v3::attributes::SERVICE_DEVICE_INFORMATION
                    && c.uuid == coyote_connect_ble::v3::attributes::CHARACTERISTIC_BATTERY
            })
            .unwrap();
        let battery = device.read(&battery_char).await.unwrap();
        println!("Battery level: {:?}", battery);
    }
}
