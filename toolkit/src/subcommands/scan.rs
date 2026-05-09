use std::{io::Write, pin::pin, time::Duration};

use futures::StreamExt;
use tokio::io::AsyncBufReadExt;

use crate::subcommands::common::{DeviceFilter, PAWPRINT_CONNECT_CMD};

pub async fn scan_coyote() -> Result<(), Box<dyn std::error::Error>> {
    scan_ble_device(
        DeviceFilter::ByName(coyote_connect_ble::v3::PULSE_HOST_DEFAULT_NAME.to_string()),
        None,
        Duration::from_secs(12),
    )
    .await
}

pub async fn scan_pawprint() -> Result<(), Box<dyn std::error::Error>> {
    scan_ble_device(
        DeviceFilter::ByName(coyote_connect_ble::pawprint::PAWPRINT_DEFAULT_NAME.to_string()),
        Some(&PAWPRINT_CONNECT_CMD),
        Duration::from_secs(2),
    )
    .await
}

pub async fn scan_ble_device(
    device_filter: DeviceFilter,
    connection_command: Option<&[u8]>,
    timeout: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    let (_, device, command_characteristic, notify_characteristic) =
        crate::subcommands::common::setup_and_discover_device(
            None,
            device_filter,
            connection_command,
        )
        .await?;

    let mut notification_stream = pin!(notify_characteristic.notify().await?);

    println!("Target device connected and ready - press Enter to start testing commands...");
    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();
    stdin.next_line().await?;

    let skip_commands = vec![
        // 0x01, 0x08, 0x0C, 0x0D, 0x0E, 0x11, 0x12, 0x20, 0x28, 0x30, 0x40, 0x50, 0x60, 0x72, 0xA0, 0xA1,
        // 0xAF, 0xB0, 0xBC, 0xBF, 0xED, 0xFE,
        // 0xFF,
    ];

    for command in (0x00..=0xFF).rev() {
        if skip_commands.contains(&command) {
            println!("0x{:02X}: [skipped]", command);
            continue;
        }

        print!("0x{:02X}: ", command);
        std::io::stdout().flush()?;

        command_characteristic.write(&[command]).await?;

        let mut received_any = false;
        loop {
            let timeout_duration = if received_any {
                Duration::from_secs(1)
            } else {
                timeout
            };

            match tokio::time::timeout(timeout_duration, notification_stream.next()).await {
                Ok(Some(data)) => {
                    if !received_any {
                        println!();
                        received_any = true;
                    }
                    println!("  -> {:02X?}", data);
                }
                Ok(None) => {
                    println!("[stream closed]");
                    break;
                }
                Err(_) => {
                    if !received_any {
                        println!("[timeout]");
                    }
                    break;
                }
            }
        }
    }

    device.disconnect().await?;

    Ok(())
}
