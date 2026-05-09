use bluer::{
    Uuid, adv::{Advertisement, Type as AdvType}, gatt::{
        local::{
            Application, Characteristic, CharacteristicNotify, CharacteristicNotifyMethod,
            CharacteristicRead, CharacteristicWrite, CharacteristicWriteMethod, Service,
        },
        remote,
    }
};
use coyote_connect_ble::pawprint::attributes::SERVICE_GENERIC_ATTRIBUTE_PROFILE;
use futures::{FutureExt, StreamExt};
use std::collections::BTreeMap;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::subcommands::common::{DeviceFilter, PAWPRINT_CONNECT_CMD};

pub async fn mitm_coyote() -> Result<(), Box<dyn std::error::Error>> {
    mitm_ble_device(
        None,
        DeviceFilter::ByName(coyote_connect_ble::v3::PULSE_HOST_DEFAULT_NAME.to_string()),
        None,
    )
    .await
}

pub async fn mitm_pawprint() -> Result<(), Box<dyn std::error::Error>> {
    mitm_ble_device(
        None,
        DeviceFilter::ByName(coyote_connect_ble::pawprint::PAWPRINT_DEFAULT_NAME.to_string()),
        Some(&PAWPRINT_CONNECT_CMD),
    )
    .await
}

pub async fn mitm_ble_device(
    adapter_filter: Option<&str>,
    device_filter: DeviceFilter,
    connection_command: Option<&[u8]>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (adapter, device, _, __) = crate::subcommands::common::setup_and_discover_device(
        adapter_filter,
        device_filter,
        connection_command,
    )
    .await?;

    println!("Target device connected and ready - press Enter to set up the proxy device...");
    let mut stdin = BufReader::new(tokio::io::stdin()).lines();
    stdin.next_line().await?;

    // Set up a proxy device with identical services/characteristics and forward reads/writes/notifications between the real device and any clients.
    let advertisement = Advertisement {
        advertisement_type: AdvType::Peripheral,
        advertising_data: device
            .advertising_data()
            .await?
            .map(|m| m.into_iter().collect())
            .unwrap_or_else(BTreeMap::new),
        appearance: device.appearance().await?,
        discoverable: Some(true),
        local_name: device.name().await?,
        manufacturer_data: device
            .manufacturer_data()
            .await?
            .map(|m| m.into_iter().collect())
            .unwrap_or_else(BTreeMap::new),
        max_interval: None,
        min_interval: None,
        service_data: device
            .service_data()
            .await?
            .map(|m| m.into_iter().collect())
            .unwrap_or_else(BTreeMap::new),
        ..Default::default()
    };
    let adv_handle = adapter.advertise(advertisement).await?;

    let proxy_device_services = device.services().await?;
    let mut proxy_device_services = Vec::with_capacity(proxy_device_services.len());
    for service in device.services().await? {
        if service.uuid().await? != SERVICE_GENERIC_ATTRIBUTE_PROFILE {
            proxy_device_services.push(service);
        }
    }
    let mut proxy_services = Vec::with_capacity(proxy_device_services.len());
    for service in proxy_device_services {
        let characteristics = service.characteristics().await?;
        let mut proxy_characteristics = Vec::with_capacity(characteristics.len());
        for characteristic in characteristics {
            let properties = characteristic.flags().await?;
            let uuid = characteristic.uuid().await?;
            let read_char = characteristic.clone();
            let write_char = characteristic.clone();
            let notify_char = characteristic.clone();
            let proxy_characteristic = Characteristic {
                uuid: uuid,
                read: if properties.read
                    || properties.encrypt_authenticated_read
                        | properties.encrypt_read
                        | properties.secure_read
                {
                    Some(CharacteristicRead {
                        read: properties.read,
                        encrypt_authenticated_read: properties.encrypt_authenticated_read,
                        encrypt_read: properties.encrypt_read,
                        secure_read: properties.secure_read,
                        fun: Box::new(move |_request| {
                            let real_characteristic = read_char.clone();
                            async move {
                                let response = real_characteristic
                                    .read()
                                    .await
                                    .map_err(|_| bluer::gatt::local::ReqError::Failed);
                                match response {
                                    Ok(ref value) => {
                                        println!(
                                            "READ {}: {}",
                                            uuid_display(&uuid),
                                            byte_display(&value)
                                        );
                                    }
                                    Err(e) => {
                                        eprintln!("READ {} Error: {}", uuid_display(&uuid), e);
                                    }
                                };
                                response
                            }
                            .boxed()
                        }),
                        ..Default::default()
                    })
                } else {
                    None
                },
                write: if properties.write
                    || properties.write_without_response
                    || properties.encrypt_authenticated_write
                    || properties.encrypt_write
                    || properties.secure_write
                    || properties.authenticated_signed_writes
                    || properties.reliable_write
                {
                    Some(CharacteristicWrite {
                        write: properties.write,
                        write_without_response: properties.write_without_response,
                        encrypt_authenticated_write: properties.encrypt_authenticated_write,
                        encrypt_write: properties.encrypt_write,
                        secure_write: properties.secure_write,
                        authenticated_signed_writes: properties.authenticated_signed_writes,
                        reliable_write: properties.reliable_write,
                        method: CharacteristicWriteMethod::Fun(Box::new(move |value, request| {
                            let real_characteristic = write_char.clone();
                            let remote_request = remote::CharacteristicWriteRequest {
                                offset: request.offset,
                                op_type: request.op_type,
                                prepare_authorize: request.prepare_authorize,
                                ..Default::default()
                            };
                            async move {
                                println!("WRITE {}: {}", uuid_display(&uuid), byte_display(&value));
                                real_characteristic
                                    .write_ext(&value, &remote_request)
                                    .await
                                    .map_err(|_| bluer::gatt::local::ReqError::Failed)
                            }
                            .boxed()
                        })),
                        ..Default::default()
                    })
                } else {
                    None
                },
                notify: if properties.notify {
                    Some(CharacteristicNotify {
                        indicate: properties.indicate,
                        notify: properties.notify,
                        method: CharacteristicNotifyMethod::Fun(Box::new(move |mut notifier| {
                            let real_characteristic = notify_char.clone();
                            async move {
                                println!("SUBSCRIBE {}", uuid_display(&uuid));
                                let stream = match real_characteristic.notify().await {
                                    Ok(stream) => stream,
                                    Err(e) => {
                                        eprintln!("Failed to subscribe to notifications on real device: {}", e);
                                        return;
                                    }
                                };
                                let mut notification_stream = Box::pin(stream);
                                while let Some(notification) = notification_stream.next().await {
                                    println!("NOTIFY {}: {}", uuid_display(&uuid), byte_display(&notification));
                                    match notifier.notify(notification).await {
                                        Ok(_) => (),
                                        Err(e) => {
                                            eprintln!("Failed to send notification to client: {}", e);
                                            break;
                                        }
                                    }
                                }
                            }
                            .boxed()
                        })),
                        ..Default::default()
                    })
                } else {
                    None
                },
                ..Default::default()
            };
            proxy_characteristics.push(proxy_characteristic);
        }
        let proxy_service = Service {
            uuid: service.uuid().await?,
            primary: service.primary().await?,
            characteristics: proxy_characteristics,
            ..Default::default()
        };
        proxy_services.push(proxy_service);
    }

    let app = Application {
        services: proxy_services,
        ..Default::default()
    };

    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("BLE proxy running. Press Ctrl+C to stop.");
    tokio::signal::ctrl_c().await?;

    println!("Shutting down...");
    drop(app_handle);
    drop(adv_handle);
    let address = device.address();
    device.disconnect().await?;
    adapter.remove_device(address).await?;

    println!("You may need to turn bluetooth off and back on again before you can connect to the device again.");
    println!("Goodbye");
    Ok(())
}

const BLUETOOTH_BASE_UUID: u128 = 0x00000000_0000_1000_8000_00805f9b34fb;
const BLUETOOTH_BASE_MASK: u128 = 0x00000000_ffff_ffff_ffff_ffffffffffff;
const BLUETOOTH_BASE_MASK_16: u128 = 0xffff0000_ffff_ffff_ffff_ffffffffffff;

fn uuid_display(uuid: &Uuid) -> String {
    if uuid.as_u128() & BLUETOOTH_BASE_MASK_16 == BLUETOOTH_BASE_UUID {
        format!("{:04x}", uuid.as_u128() >> 96)
    } else if uuid.as_u128() & BLUETOOTH_BASE_MASK == BLUETOOTH_BASE_UUID {
        format!("{:08x}", uuid.as_u128() >> 96)
    } else {
        uuid.to_string()
    }
}

fn byte_display(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
