use bluer::{Adapter, AdapterEvent, Address, Device, gatt::remote::Characteristic};
use coyote_connect_ble::pawprint::attributes::{
    CHARACTERISTIC_DEVICE_COMMAND, CHARACTERISTIC_DEVICE_NOTIFICATION, SERVICE_PULSE_HOST,
};
use futures::StreamExt;
use tokio::io::{AsyncBufReadExt, BufReader};

pub enum DeviceFilter {
    ByName(String),
    #[allow(dead_code)]
    ByAddress(Address),
}

pub const PAWPRINT_CONNECT_CMD: [u8; 17] = [
    0x50, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00,
];

pub async fn setup_and_discover_device(
    adapter_filter: Option<&str>,
    device_filter: DeviceFilter,
    connection_command: Option<&[u8]>,
) -> Result<(Adapter, Device, Characteristic, Characteristic), Box<dyn std::error::Error>> {
    println!(
        "Make sure the target device is discoverable and in range, then press Enter to continue..."
    );
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    stdin.next_line().await?;

    let session = bluer::Session::new().await?;
    let adapter = match adapter_filter {
        Some(name) => session.adapter(name)?,
        None => session.default_adapter().await?,
    };
    println!(
        "Using adapter \"{}\" ({})",
        adapter.name(),
        adapter.address().await?
    );

    println!("Discovering devices...");
    let mut discover_stream = adapter.discover_devices().await?;
    let mut device = None;
    while let Some(evt) = discover_stream.next().await {
        if let AdapterEvent::DeviceAdded(addr) = evt {
            let added_device = adapter.device(addr)?;
            match &device_filter {
                DeviceFilter::ByName(name) => {
                    if added_device.name().await?.is_some_and(|n| n == *name) {
                        device = Some(added_device);
                        break;
                    }
                }
                DeviceFilter::ByAddress(filter_addr) => {
                    if added_device.address() == *filter_addr {
                        device = Some(added_device);
                        break;
                    }
                }
            }
        }
    }
    let device = device.ok_or("No matching device found")?;
    println!(
        "Found device \"{}\" ({})",
        device.name().await?.unwrap_or_default(),
        device.address()
    );

    if !device.is_connected().await? {
        println!("Connecting...");
        device.connect().await?;
        println!("Connected!");
    }

    let services = device.services().await?;
    let mut host_service = None;
    for service in services {
        if service.uuid().await? == SERVICE_PULSE_HOST {
            host_service = Some(service);
            break;
        }
    }
    let host_service = host_service.ok_or("Host service not found")?;

    let characteristics = host_service.characteristics().await?;
    let mut command_characteristic = None;
    let mut notify_characteristic = None;
    for characteristic in characteristics {
        let uuid = characteristic.uuid().await?;
        if uuid == CHARACTERISTIC_DEVICE_COMMAND {
            command_characteristic = Some(characteristic);
        } else if uuid == CHARACTERISTIC_DEVICE_NOTIFICATION {
            notify_characteristic = Some(characteristic);
        }
    }
    let command_characteristic =
        command_characteristic.ok_or("Command characteristic not found")?;
    let notify_characteristic = notify_characteristic.ok_or("Notify characteristic not found")?;

    if let Some(cmd) = connection_command {
        println!("Executing connection command...");
        command_characteristic.write(cmd).await?;
    }

    Ok((
        adapter,
        device,
        command_characteristic,
        notify_characteristic,
    ))
}
