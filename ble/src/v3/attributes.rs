//! Bluetooth attributes
//! 
//! [0x1800]: #0x1800---gap-service
//! [0x1801]: #0x1801---generic-attribute-profile-gatt-service
//! [0x180A]: #0x180a---device-information-service
//! [0x180C]: #0x180c---pulse-host-service
//! [0x2003]: #0x2003---unknown-service
//! [0x2004]: #0x2004---unknown-service
//! [0xFE59]: #0xfe59---secure-dfu-service
//! 
//! [0xFE59]: #0xfe59---secure-dfu-service
//! [0x2A00]: #0x2a00---device-name-characteristic
//! [0x2A01]: #0x2a01---appearance-characteristic
//! [0x2A04]: #0x2a04---peripheral-preferred-connection-parameters-characteristic
//! [0x2AA6]: #0x2aa6---central-address-resolution-characteristic
//! [0x2A05]: #0x2a05---service-changed-characteristic
//! [0x1500]: #0x1500---battery-level-characteristic
//! [0x1501]: #0x1501---version-characteristic
//! [0x1502]: #0x1502---bluetooth-mac-characteristic
//! [0x2A59]: #0x2a59---unknown-characteristic
//! [0x180C]: #0x180c---pulse-host-characteristic
//! [0x150A]: #0x150a---device-command-characteristic
//! [0x150B]: #0x150b---device-notification-characteristic
//! [0x0007]: #0x0007---unknown-characteristic
//! [0x0008]: #0x0008---unknown-characteristic
//! [0x0009]: #0x0009---unknown-characteristic
//! [8ec90003-f315-4f60-9fb8-838830daea50]: #8ec90003-f315-4f60-9fb8-838830daea50---buttonless-dfu
//! 
//! ## Attribute Table
//! 
//! | Service UUID | Service Name                         | Characteristic UUID                  | Characteristic Name                                    | Properties           |
//! |--------------|--------------------------------------|--------------------------------------|--------------------------------------------------------|----------------------|
//! | 0x1800       | [GAP Service][0x1800]                | 0x2A00                               | [Device Name][0x2A00]                                  | Read, Write          |
//! | 0x1800       | [GAP Service][0x1800]                | 0x2A01                               | [Appearance][0x2A01]                                   | Read                 |
//! | 0x1800       | [GAP Service][0x1800]                | 0x2A04                               | [Peripheral Preferred Connection Parameters][0x2A04]   | Read                 |
//! | 0x1800       | [GAP Service][0x1800]                | 0x2AA6                               | [Central Address Resolution][0x2AA6]                   | Read                 |
//! | 0x1801       | [GATT Service][0x1801]               | 0x2A05                               | [Service Changed][0x2A05]                              | Indicate             |
//! | 0x180A       | [Device Information Service][0x180A] | 0x1500                               | [Battery Level][0x1500]                                | Read, Notify         |
//! | 0x180A       | [Device Information Service][0x180A] | 0x1501                               | [Version][0x1501]                                      | Read                 |
//! | 0x180A       | [Device Information Service][0x180A] | 0x1502                               | [Bluetooth MAC][0x1502]                                | Read                 |
//! | 0x180A       | [Device Information Service][0x180A] | 0x2A59                               | [Unknown Characteristic][0x2A59]                       | Read, Notify         |
//! | 0x180C       | [Pulse Host Service][0x180C]         | 0x150A                               | [Device Command][0x150A]                               | Write No Response    |
//! | 0x180C       | [Pulse Host Service][0x180C]         | 0x150B                               | [Device Notification][0x150B]                          | Notify               |
//! | 0x2003       | [Unknown Service][0x2003]            | 0x0007                               | [Unknown Characteristic][0x0007]                       | Write                |
//! | 0x2003       | [Unknown Service][0x2003]            | 0x0008                               | [Unknown Characteristic][0x0008]                       | Read, Notify         |
//! | 0x2004       | [Unknown Service][0x2004]            | 0x0009                               | [Unknown Characteristic][0x0009]                       | Read                 |
//! | 0xFE59       | [Secure DFU Service][0xFE59]         | 8ec90003-f315-4f60-9fb8-838830daea50 | [Buttonless DFU][8ec90003-f315-4f60-9fb8-838830daea50] | Write, Indicate      |
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
//! # 0x180A - Device Information Service 
//! Manufacturer/vendor information about the device.
//! 
//! > NOTE: DG-Labs does not use any of the standard characteristics for this service. All characteristics are custom.
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
//! #### Examples
//! - `0x64` - 100%
//! - `0x4A` - 74%
//! - `0x32` - 50%
//! - `0x00` - 0%
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
//! > NOTE: I haven't verified this, but the app has code that suggests v2 devices have the version and label flipped (see examples).
//! 
//! #### Examples
//! - `0x09 0x03` - DG Labs Coyote 3.0, firmware vers 9
//! - `0x07 0x03` - DG Labs Coyote 3.0, firmware vers 7
//! - `0x02 0x01` - DG Labs Coyote 2.0
//! - `0x02 0x10` - DG Labs Coyote 2.0
//! 
//! ### 0x1502 - Bluetooth MAC Characteristic
//! The Bluetooth MAC address of the device.
//! 
//! #### Properties
//! Read
//! 
//! #### Value
//! 6 bytes - The MAC address, as bytes, in big-endian order.
//! 
//! #### Examples
//! - `0xD4 0xF5 0x13 0x2A 0xB4 0x6C` - MAC address `D4:F5:13:2A:B4:6C`
//! - `0xC4 0x7C 0x8D 0x99 0x5E 0xD0` - MAC address `C4:7C:8D:99:5E:D0`
//! 
//! ### 0x2A59 - Unknown Characteristic
//! Undocumented characteristic with unknown purpose.
//! 
//! > Notes:
//! > - I don't remember seeing this characteristic before - I haven't updated the firmware so I'm not sure if it was hiding or I just missed it last time.
//! > - This id (0x2A59) is in nordic's numbers database as "Analog Output" from gss - I assume that's the GATT Specification Supplement but it doesn't mention this characteristic anywhere.
//! > - The xiao esp32c6 documentation shows using service 0x181A (environmental monitoring) and characteristic 0x2A59 for reporting Analog Output
//!
//! #### Properties
//! Read, Notify
//! 
//! #### Value
//! Unknown - Always 10 bytes long, and updates every 4 seconds.
//!
//! The first two bytes match the load detection indicator in the app for the left and right channels.
//! - 0x01 - Inactive
//! - 0x02 - Connected
//! - 0x03 - Disconnected
//! 
//! The third and forth bytes are unknown but appear to be related - one for the left channel and one for the right channel.
//! 
//! The fifth and sixth bytes are unknown but appear to be related - one for the left channel and one for the right channel.
//! 
//! The last four bytes are unknown.
//! 
//! ```
//! |      0      |      1      |      2      |      3      |      4      |      5      |      6      |      7      |      8      |      9      |
//! +-------------+-------------+-------------+-------------+-------------+-------------+-------------+-------------+-------------+-------------+
//! | Left Conn   | Right Conn  | Left ?      | Right ?     | Left ?      | Right ?     | ?           | ?           | ?           | ?           |
//! +-------------+-------------+-------------+-------------+-------------+-------------+-------------+-------------+-------------+-------------+
//! ```
//! 
//! #### Examples
//! - `0x01 0x01 0x00 0x00 0x00 0x00 0xF4 0xF4 0xF4 0xF4` - Device Idle
//! - `0x01 0x01 0x00 0x00 0x00 0x00 0xA0 0xA0 0xA0 0xA0` - Device Idle
//! - `0x01 0x01 0x00 0x00 0x00 0x00 0xE0 0x32 0x32 0x32` - Device Idle
//! - `0x02 0x01 0x8C 0x00 0x64 0x00 0x64 0x84 0x74 0x74` - Device Active left
//! - `0x02 0x01 0x8C 0x00 0x78 0x00 0x64 0x42 0x32 0x32` - Device Active left
//! - `0x02 0x01 0x8C 0x00 0x6E 0x00 0x64 ...` - Device Active left
//! - `0x02 0x01 0x8C 0x00 0x64 0x00 0x64 ...` - Device Active left
//! - `0x01 0x02 0x00 0x8C 0x00 0x78 0xCE 0x64 0x20 0x20` - Device Active right
//! - `0x01 0x02 0x00 0x8C 0x00 0x78 0x38 0x64 0x8A 0x8A` - Device Active right
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
//! # 0x2003 - Unknown Service
//!! Undocumented service with unknown purpose.
//! 
//! ### 0x0007 - Unknown Characteristic
//! Undocumented characteristic with unknown purpose.
//! 
//! #### Properties
//! Write
//! 
//! ### 0x0008 - Unknown Characteristic
//! Undocumented characteristic with unknown purpose.
//! 
//! #### Properties
//! Read, Notify
//! 
//! # 0x2004 - Unknown Service
//! Undocumented service with unknown purpose.
//! 
//! ## 0x0009 - Unknown Characteristic
//! Undocumented characteristic with unknown purpose.
//! 
//! #### Properties
//! Read
//! 
//! # 0xFE59 - Secure DFU Service
//! Nordic's proprietary secure Device Firmware Update (DFU) service.
//! 
//! - <https://docs.nordicsemi.com/bundle/sdk_nrf5_v17.0.2/page/service_dfu.html>
//! 
//! ### 8ec90003-f315-4f60-9fb8-838830daea50 - Buttonless DFU Characteristic
//! Characteristic for initiating Buttonless DFU without bonds.
//! 
//! #### Properties
//! Write, Indicate
//!

use btleplug::api::bleuuid::uuid_from_u16;
use uuid::Uuid;

pub const SERVICE_GENERIC_ACCESS_PROFILE: Uuid = uuid_from_u16(0x1800);
pub const CHARACTERISTIC_DEVICE_NAME: Uuid = uuid_from_u16(0x2A00);
pub const CHARACTERISTIC_APPEARANCE: Uuid = uuid_from_u16(0x2A01);
pub const CHARACTERISTIC_PREFERRED_CONNECTION_PARAMETERS: Uuid = uuid_from_u16(0x2A04);
pub const CHARACTERISTIC_CENTRAL_ADDRESS_RESOLUTION: Uuid = uuid_from_u16(0x2AA6);

pub const SERVICE_GENERIC_ATTRIBUTE_PROFILE: Uuid = uuid_from_u16(0x1801);
pub const CHARACTERISTIC_SERVICE_CHANGED: Uuid = uuid_from_u16(0x2A05);

pub const SERVICE_DEVICE_INFORMATION: Uuid = uuid_from_u16(0x180A);
pub const CHARACTERISTIC_BATTERY: Uuid = uuid_from_u16(0x1500);
pub const CHARACTERISTIC_VERSION: Uuid = uuid_from_u16(0x1501);
pub const CHARACTERISTIC_MAC: Uuid = uuid_from_u16(0x1502);
pub const CHARACTERISTIC_UNKNOWN_2A59: Uuid = uuid_from_u16(0x2A59);

pub const SERVICE_PULSE_HOST: Uuid = uuid_from_u16(0x180C);
pub const CHARACTERISTIC_DEVICE_COMMAND: Uuid = uuid_from_u16(0x150A);
pub const CHARACTERISTIC_DEVICE_NOTIFICATION: Uuid = uuid_from_u16(0x150B);

pub const SERVICE_UNKNOWN_2003: Uuid = uuid_from_u16(0x2003);
pub const CHARACTERISTIC_UNKNOWN_0007: Uuid = uuid_from_u16(0x0007);
pub const CHARACTERISTIC_UNKNOWN_0008: Uuid = uuid_from_u16(0x0008);

pub const SERVICE_UNKNOWN_2004: Uuid = uuid_from_u16(0x2004);
pub const CHARACTERISTIC_UNKNOWN_0009: Uuid = uuid_from_u16(0x0009);

pub const SERVICE_SECURE_DFU: Uuid = uuid_from_u16(0xFE59);
pub const CHARACTERISTIC_BUTTONLESS_DFU: Uuid = Uuid::from_u128(0x8ec90003f3154f609fb8838830daea50);
