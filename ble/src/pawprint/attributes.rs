//! Bluetooth attributes
//!
//! ## Attribute Table
//!
//! | Service                     | Characteristic                      | Properties   | Value Size      | Description                                               |      Example Value |
//! |-----------------------------|-------------------------------------|--------------|-----------------|-----------------------------------------------------------|--------------------|
//! | 0x1800 - Generic Access     | 0x2A00 - Device Name                | Read, Write  | Up to 248 bytes | Device name                                               |        "47L120100" |
//! | 0x1801 - Generic Attribute  | 0x2A05 - Service Changed            | Read, Indicate | 4 bytes       | Service Changed                                           |         0x0100FFFF |
//! | 0x180C - PawPrint           | 0x150A - Device Command             | Write        | Up to 20 bytes  | All commands are input in this characteristic             |                    |
//! | 0x180C - PawPrint           | 0x150B - Device Notification        | Read, Notify | Up to 20 bytes  | All response messages are returned in this characteristic |                    |
//! | F000FFC0-0451-4000-B000-000000000000 - TI SensorTag OvertheAir Download | F000FFC1-0451-4000-B000-000000000000 - Image Identify | Write No Response, Write, Notify | Unknown | TODO: read TI docs | |
//! | F000FFC0-0451-4000-B000-000000000000 - TI SensorTag OvertheAir Download | F000FFC2-0451-4000-B000-000000000000 - Img Block | Write No Response, Write, Notify | Unknown | TODO: read TI docs | |
//!
//! # 0x1800 - GAP Service
//! Generic Access Profile service that contains the device name, appearance, and connection parameters.
//!
//! - [Bluetooth Core 6.0 | Volume 3. Host | Part C. Generic Access Profile | 12. GAP service and characteristics for GATT Server](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-37d5043e-0d5b-e174-a1fa-91006b88a3db)
//!
//! ### 0x2A00 - Device Name Characteristic
//! The name of the device. This is the name that is displayed to users.
//!
//! - [Bluetooth Core 6.0 | Volume 3. Host | Part C. Generic Access Profile | 12.1. Device Name characteristic](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-9cd6789b-32b0-e9bf-7426-b3eeda194156)
//! - [Bluetooth Core 6.0 | Volume 3. Host | Part C. Generic Access Profile | 3.2.2. Bluetooth Device Name (the user-friendly name)](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-e91f7086-ea42-e034-0e8a-82da5981c8cf)
//!
//! > Notes:
//! > - This characteristic is writable, but changing the name is not persistent and only affects reads of this characteristic until the device is restarted.
//! > - The name shown in the OS's Bluetooth settings are not affected by changes to this characteristic.
//! > - It does not appear to be shown anywhere in the app.
//!
//! #### Properties
//!
//! Read, Write
//!
//! #### Value
//!
//! UTF-8 formated string, up to 248 bytes long
//!
//! | Device          | Default Name  | DFU Name       |
//! |-----------------|---------------|----------------|
//! | PawPrint Button | 47L120100     | 47L120100_O3   |
//! | Coyote 3.0      | 47L121000     | (unknown)      |
//! | Coyote 2.0      | D-LAB ESTIM01 | (unknown)      |
//!
//! # 0x1801 - Generic Attribute Profile (GATT) Service
//! Standard service that contains the Service Changed characteristic.
//!
//!  - [Bluetooth Core 6.0 | Volume 3. Host | Part G. Generic Attribute Profile (GATT) | 7. Defined Generic Attribute Profile service](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-d551badd-2aae-c2e1-8722-a5f8bcd79600)
//!
//! ### 0x2A05 - Service Changed Characteristic
//! Indicates that a service has changed.
//!
//!  - [Bluetooth Core 6.0 | Volume 3. Host | Part G. Generic Attribute Profile (GATT) | 7.1. Service Changed](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-attribute-profile--gatt-.html#UUID-90b8f0f4-3b5c-7c4c-061c-6d981473335c)
//!
//! #### Properties
//! Indicate
//!
//! #### Value
//! 4 bytes
//!
//! ```
//! |    0     |    1     |    2     |    3     |
//! +----------+----------+----------+----------+
//! | Start Handle        | End Handle          |
//! +----------+----------+----------+----------+
//! ```
//!
//! # 0x180C - Pulse Host Service
//! Service for communication with the Coyote device.
//!
//! ### 0x150A - Device Command Characteristic
//! Characteristic for sending commands to the device.
//!
//! #### Properties
//! Write No Response
//!
//! #### Value
//! Variable length byte array. First byte is the command ID, followed by command-specific data.
//!
//! ### 0x150B - Device Notification Characteristic
//! Characteristic for receiving notifications from the device.
//!
//! #### Properties
//! Notify
//!
//! #### Value
//! Variable length byte array. First byte is the notification ID, followed by notification-specific data.
//!
//! # 0xF000FFC0-0451-4000-B000-000000000000 - TI SensorTag OAD Service
//! TI's Over the Air Download service used for firmware updates.
//!
//! - https://software-dl.ti.com/lprf/simplelink_cc2640r2_sdk/1.35.00.33/exports/docs/ble5stack/ble_user_guide/html/oad/oad_concepts.html#oad-service-0xffc0
//!
//! ### 0xF000FFC1-0451-4000-B000-000000000000 - OAD Image Identify Characteristic
//! The Image Identify characteristic is used to exchange image metadata between OAD Downloader and target.
//!
//! - https://software-dl.ti.com/lprf/simplelink_cc2640r2_sdk/1.35.00.33/exports/docs/ble5stack/ble_user_guide/html/oad/oad_concepts.html#oad-image-identify-0xffc1
//!
//! #### Properties
//! Write No Response, Write, Notify
//!
//! ### 0xF000FFC2-0451-4000-B000-000000000000 - OAD Image Block Characteristic
//! The OAD Image Block characteristic is used to request and transfer a block of the OAD image.
//!
//! - https://software-dl.ti.com/lprf/simplelink_cc2640r2_sdk/1.35.00.33/exports/docs/ble5stack/ble_user_guide/html/oad/oad_concepts.html#oad-image-block-characteristic-0xffc2
//!
//! #### Properties
//! Write No Response, Write, Notify
//!

use btleplug::api::bleuuid::uuid_from_u16;
use uuid::Uuid;

pub const SERVICE_GENERIC_ACCESS_PROFILE: Uuid = uuid_from_u16(0x1800);
pub const CHARACTERISTIC_DEVICE_NAME: Uuid = uuid_from_u16(0x2A00);

pub const SERVICE_GENERIC_ATTRIBUTE_PROFILE: Uuid = uuid_from_u16(0x1801);
pub const CHARACTERISTIC_SERVICE_CHANGED: Uuid = uuid_from_u16(0x2A05);

pub const SERVICE_PULSE_HOST: Uuid = uuid_from_u16(0x180C);
pub const CHARACTERISTIC_DEVICE_COMMAND: Uuid = uuid_from_u16(0x150A);
pub const CHARACTERISTIC_DEVICE_NOTIFICATION: Uuid = uuid_from_u16(0x150B);

pub const TI_BLUETOOTH_BASE_UUID: u128 = 0xF0000000_0451_4000_B000_000000000000;
pub const fn ti_uuid_from_u32(short: u32) -> Uuid {
    Uuid::from_u128(TI_BLUETOOTH_BASE_UUID | ((short as u128) << 96))
}

pub const SERVICE_OAD_DOWNLOAD: Uuid = ti_uuid_from_u32(0xFFC0);
pub const CHARACTERISTIC_IMAGE_IDENTIFY: Uuid = ti_uuid_from_u32(0xFFC1);
pub const CHARACTERISTIC_IMAGE_BLOCK: Uuid = ti_uuid_from_u32(0xFFC2);
