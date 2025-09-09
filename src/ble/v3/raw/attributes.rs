//! Bluetooth attributes
//! 
//! ## Attribute Table
//! - [0x1800 - GAP Service](#0x1800---gap-service)
//!   - [0x2A00 - Device Name](#0x2A00---device-name-characteristic)
//!   - [0x2A01 - Appearance](#0x2A01---appearance-characteristic)
//!   - [0x2A04 - Peripheral Preferred Connection Parameters](#0x2A04---peripheral-preferred-connection-parameters-characteristic)
//!   - [0x2AA6 - Central Address Resolution](#0x2AA6---central-address-resolution-characteristic)
//! - [0x1801 - GATT Service](#0x1801---generic-attribute-profile-gatt-service)
//!   - [0x2A05 - Service Changed](#0x2A05---service-changed-characteristic)
//! - [0x180A - Device Information Service](#0x180A---device-information-service)
//!   - [0x1500 - Battery Level](#0x1500---battery-level-characteristic)
//!   - [0x1501 - Version](#0x1501---version-characteristic)
//!   - [0x1502 - Bluetooth MAC](#0x1502---bluetooth-mac-characteristic)
//! - [0x180C - Pulse Host](#0x180C---pulse-host-characteristic)
//!   - [0x150A - Device Command](#0x150A---device-command-characteristic)
//!   - [0x150B - Device Notification](#0x150B---device-notification-characteristic)
//! - [0x2003 - Unknown Service](#0x2003---unknown-service)
//!   - [0x0007 - Unknown Characteristic](#0x0007---unknown-characteristic)
//!   - [0x0008 - Unknown Characteristic](#0x0008---unknown-characteristic)
//! - [0x2004 - Unknown Service](#0x2004---unknown-service)
//!   - [0x0009 - Unknown Characteristic](#0x0009---unknown-characteristic)
//! - [0xFE59 - Secure DFU Service](#0xFE59---secure-dfu-service)
//!   - [8ec90003-f315-4f60-9fb8-838830daea50 - Buttonless DFU](#8ec90003-f315-4f60-9fb8-838830daea50---buttonless-dfu)
//! 
//! | Service UUID | Service Name               | Characteristic UUID | Characteristic Name        | Properties           |
//! |--------------|----------------------------|---------------------|----------------------------|----------------------|
//! | 0x1800       | GAP Service                | 0x2A00              | Device Name                | Read, Write          |
//! | 0x1800       | GAP Service                | 0x2A01              | Appearance                 | Read                 |
//! | 0x1800       | GAP Service                | 0x2A04              | Peripheral Preferred Connection Parameters | Read |
//! | 0x1800       | GAP Service                | 0x2AA6              | Central Address Resolution | Read                 |
//! | 0x1801       | GATT Service               | 0x2A05              | Service Changed            | Indicate             |
//! | 0x180A       | Device Information Service | 0x1500              | Battery Level              | Read, Notify         |
//! | 0x180A       | Device Information Service | 0x1501              | Version                    | Read                 |
//! | 0x180A       | Device Information Service | 0x1502              | Bluetooth MAC              | Read                 |
//! | 0x180C       | Pulse Host                 | 0x150A              | Device Command             | Write                |
//! | 0x180C       | Pulse Host                 | 0x150B              | Device Notification        | Notify               |
//! | 0x2003       | Unknown Service            | 0x0007              | Unknown Characteristic     | Write                |
//! | 0x2003       | Unknown Service            | 0x0008              | Unknown Characteristic     | Read, Notify         |
//! | 0x2004       | Unknown Service            | 0x0009              | Unknown Characteristic     | Read                 |
//! | 0xFE59       | Secure DFU Service         | 8ec90003-f315-4f60-9fb8-838830daea50 | Buttonless DFU | Write, Indicate |
//!
//! ## 0x1800 - GAP Service
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
//! #### Properties
//! 
//! Read, Write
//! 
//! #### Value
//! 
//! UTF-8 formated string, up to 248 bytes long
//! 
//! | Device          | Default Name  |
//! |-----------------|---------------|
//! | PawPrint Button | 47L120100     |
//! | Coyote 3.0      | 47L121000     |
//! | Coyote 2.0      | D-LAB ESTIM01 |
//! 
//! 
//! ### 0x2A01 - Appearance Characteristic
//! The appearance of the device. Determines the icon that is shown to users.
//! 
//! - [Bluetooth Core 6.0 | Volume 3. Host | Part C. Generic Access Profile | 12.2. Appearance characteristic](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-ec0b9e4b-8d14-7280-a0ae-68c61f6f00eb)
//! - [Bluetooth Core 6.0 | Volume 3. Host | Part C. Generic Access Profile | 3.2.5. Appearance characteristic](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-183aa37b-6649-26a8-45dc-e167c3831775)
//! 
//! #### Properties
//! Read
//! 
//! #### Value
//! 2 bytes - The first 10 bits are the Category, the remaining 6 bits are the Subcategory.
//! 
//! ```
//! |           0           |           1           |
//! +-----------------------+-----+-----------------+
//! |15                          6|5               0|
//! +-----------------------------+-----------------+
//! | Category                    | Subcategory     |
//! +-----------------------------+-----------------+
//! ```
//! 
//! Seems to be `0x0000` for all DG-Labs devices.
//! 
//! See [Assigned Numbers | 2.6 Appearance Values](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Assigned_Numbers/out/en/Assigned_Numbers.pdf)
//! 
//! ### 0x2A04 - Peripheral Preferred Connection Parameters Characteristic
//! The preferred connection parameters.
//! 
//! - [Bluetooth Core 6.0 | Volume 3. Host | Part C. Generic Access Profile | 12.3. Peripheral Preferred Connection Parameters characteristic](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-7ef0bdcb-4c81-1aea-5f65-4a69eab5c899)
//! - [Bluetooth Core 6.0 | Volume 3. Host | Part C. Generic Access Profile | 9.3.12. Connection interval timing parameters](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-55927dc5-1fee-0edc-6e1c-91a6394c065f)
//! 
//! #### Properties
//! Read
//! 
//! #### Value
//! 8 bytes
//! 
//! ```
//! |   0    |   1    |   2    |   3    |   4    |   5    |   6    |   7    |
//! +--------+--------+--------+--------+--------+--------+--------+--------+
//! | Interval_Min    | Interval_Max    | Latency         | Timeout         |
//! +-----------------+-----------------+-----------------+-----------------+
//! ```
//! 
//! ### 0x2AA6 - Central Address Resolution Characteristic
//! Defines whether the device supports privacy with address resolution.
//! 
//! - [Bluetooth Core 6.0 | Volume 3. Host | Part C. Generic Access Profile | 12.4. Central Address Resolution characteristic](https://www.bluetooth.com/wp-content/uploads/Files/Specification/HTML/Core-60/out/en/host/generic-access-profile.html#UUID-a27e144b-e477-bda8-76aa-842727aea38f)
//! 
//! #### Properties
//! Read
//! 
//! #### Value
//! 1 byte - boolean
//! - `0x01` - true
//! - `0x00` - false
//! 
//! ```
//! |    0     |
//! +----------+
//! | Value    |
//! +----------+
//! ```
//! 
//! ## 0x1801 - Generic Attribute Profile (GATT) Service
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
//! 
//! ```
//! |    0     |    1     |    2     |    3     |
//! +----------+----------+----------+----------+
//! | Start Handle        | End Handle          |
//! +----------+----------+----------+----------+
//! ```
//! 
//! ## 0x180A - Device Information Service 
//! Manufacturer/vendor information about the device.
//! 
//! > [!NOTE]  
//! > DG-Labs does not use any of the standard characteristics for this service. All characteristics are custom.
//! 
//! - [Device Information Service 1.2](https://www.bluetooth.com/specifications/specs/dis-1-2/)
//! 
//! ### 0x1500 - Battery Level Characteristic
//! The battery level of the device.
//! 
//! #### Properties
//! Read, Notify
//! 
//! #### Value
//! 1 byte - The battery level, in percent. For example, `0x4A` is 74%.
//! 
//! ```
//! |    0     |
//! +----------+
//! | BatLevel |
//! +----------+
//! ```
//! 
//! ### 0x1501 - Version Characteristic
//! The version of the device.
//! 
//! #### Properties
//! Read
//! 
//! #### Value
//! 2 bytes
//! 
//! ```
//! |    0     |    1     |
//! +----------+----------+
//! | Version  | Label    |
//! +----------+----------+
//! ```
//! 
//! #### Examples
//! - `0x09 0x03` - DG Labs Coyote 3.0, firmware vers 9
//! - `0x07 0x03` - DG Labs Coyote 3.0, firmware vers 7
//! - `0x02 0x01` - DG Labs Coyote 2.0
//! - `0x02 0x10` - DG Labs Coyote 2.0
//! 

use btleplug::api::bleuuid::uuid_from_u16;
use uuid::Uuid;

pub const SERVICE_GENERIC_ACCESS_PROFILE: Uuid = uuid_from_u16(0x1800);
pub const SERVICE_GENERIC_ATTRIBUTE_PROFILE: Uuid = uuid_from_u16(0x1801);
pub const SERVICE_DEVICE_INFORMATION: Uuid = uuid_from_u16(0x180A);
pub const SERVICE_PULSE_HOST: Uuid = uuid_from_u16(0x180C);
pub const SERVICE_UNKNOWN_1: Uuid = uuid_from_u16(0x2003);
pub const SERVICE_UNKNOWN_2: Uuid = uuid_from_u16(0x2004);
pub const SERVICE_SECURE_DFU: Uuid = uuid_from_u16(0xFE59);

pub const CHARACTERISTIC_DEVICE_NAME: Uuid = uuid_from_u16(0x2A00);
pub const CHARACTERISTIC_APPEARANCE: Uuid = uuid_from_u16(0x2A01);
pub const CHARACTERISTIC_PREFERRED_CONNECTION_PARAMETERS: Uuid = uuid_from_u16(0x2A04);
pub const CHARACTERISTIC_CENTRAL_ADDRESS_RESOLUTION: Uuid = uuid_from_u16(0x2AA6);
pub const CHARACTERISTIC_SERVICE_CHANGED: Uuid = uuid_from_u16(0x2A05);
pub const CHARACTERISTIC_BATTERY: Uuid = uuid_from_u16(0x1500);
pub const CHARACTERISTIC_UNKNOWN_DEVICE_INFORMATION_1: Uuid = uuid_from_u16(0x1501);
pub const CHARACTERISTIC_UNKNOWN_DEVICE_INFORMATION_2: Uuid = uuid_from_u16(0x1502);
pub const CHARACTERISTIC_DEVICE_COMMAND: Uuid = uuid_from_u16(0x150A);
pub const CHARACTERISTIC_DEVICE_NOTIFICATION: Uuid = uuid_from_u16(0x150B);
pub const CHARACTERISTIC_UNKNOWN_1: Uuid = uuid_from_u16(0x0007);
pub const CHARACTERISTIC_UNKNOWN_2: Uuid = uuid_from_u16(0x0008);
pub const CHARACTERISTIC_UNKNOWN_3: Uuid = uuid_from_u16(0x0009);
pub const CHARACTERISTIC_BUTTONLESS_DFU: Uuid = Uuid::from_u128(0x8ec90003f3154f609fb8838830daea50);
