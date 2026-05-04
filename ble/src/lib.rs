use btleplug::api::{Central, Peripheral, ScanFilter};
use btleplug::platform::Adapter;

pub mod pawprint;
pub mod v2;
pub mod v3;

pub enum CoyoteBleDevice<P: Peripheral> {
    // V2(v2::CoyoteV2Device),
    V3(v3::device::CoyoteV3Device<P>),
    // PawPrint(pawprint::CoyotePawPrintDevice),
}

pub fn default_filter() -> ScanFilter {
    ScanFilter {
        services: vec![
            // v2::attributes::SERVICE_PULSE_HOST,
            v3::attributes::SERVICE_PULSE_HOST,
            // pawprint::attributes::PAWPRINT_HOST,
        ],
    }
}

/// Scans for a Coyote BLE device and returns the first one found.
pub async fn find_device_oneshot(
    adapter: &Adapter,
    timeout: std::time::Duration,
) -> Result<Option<CoyoteBleDevice<impl Peripheral>>, btleplug::Error> {
    let start = std::time::Instant::now();
    adapter.start_scan(default_filter()).await?;
    let device = 'outer: loop {
        let peripherals = adapter.peripherals().await?;
        for peripheral in peripherals {
            if let Some(device) = device_for_peripheral(peripheral).await? {
                break 'outer Some(device);
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        if start.elapsed() > timeout {
            break 'outer None;
        }
    };
    adapter.stop_scan().await?;
    Ok(device)
}

/// Attempts to create a `CoyoteBleDevice` from the given peripheral, returning `Ok(None)` if the peripheral is not a Coyote device.
pub async fn device_for_peripheral<P: Peripheral>(
    peripheral: P,
) -> Result<Option<CoyoteBleDevice<P>>, btleplug::Error> {
    let properties = match peripheral.properties().await? {
        Some(props) => props,
        None => return Ok(None),
    };
    if properties
        .services
        .iter()
        .any(|s| s == &v3::attributes::SERVICE_PULSE_HOST)
        && properties
            .local_name
            .is_some_and(|name| name == v3::PULSE_HOST_DEFAULT_NAME)
    {
        let is_connected = peripheral.is_connected().await?;
        if !is_connected {
            peripheral.connect().await?;
        }
        let is_connected = peripheral.is_connected().await?;
        if !is_connected {
            return Err(btleplug::Error::NotConnected);
        }
        return Ok(Some(CoyoteBleDevice::V3(v3::device::CoyoteV3Device::new(
            peripheral,
        ))));
    }
    Ok(None)
}
