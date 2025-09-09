//! Pulse Host notifications
//!
//! Notifications are formatted as a single byte identifier followed by a variable length payload.
//!
//! ```
//! +-----+------+------+------+...>
//! | CMD | DATA                   |
//! +-----+------+------+------+...>
//! ```
//!
//! | Notification | Length   | Description                                                                         | Data Example         |
//! |--------------|----------|-------------------------------------------------------------------------------------|----------------------|
//! | 0x02         | 5 bytes  | Response to 0x01 command?                                                           | 0x02 01 01 0a 01     |
//! | 0x03         | Unknown  | Unknown                                                                             |                      |
//! | 0x04         | Unknown  | Unknown                                                                             |                      |
//! | 0x09         | 2 bytes  | Response to 0x08 command? (dupe body data)                                          | 0x09 01              |
//! | 0x0C         | Unknown  | Unknown                                                                             |                      |
//! | 0x0D         | Unknown  | Unknown                                                                             |                      |
//! | 0x0E         | Unknown  | Unknown                                                                             |                      |
//! | 0x0F         | Unknown  | Unknown                                                                             |                      |
//! | 0x10         | Unknown  | Unknown                                                                             |                      |
//! | 0x21         | Unknown  | Unknown                                                                             |                      |
//! | 0x29         | Unknown  | Unknown                                                                             |                      |
//! | 0x31         | Unknown  | Unknown                                                                             |                      |
//! | 0x41         | Unknown  | Unknown                                                                             |                      |
//! | 0x51         | 4 bytes  | Last byte is same as battery level                                                  | 0x51 00 10 5B        |
//! | 0x53         | 6 bytes  | Unknown                                                                             | 0x53 00 39 06 2B F8  |
//! | 0x61         | 4 bytes  | Response to 0x60 command? (dupe body data)                                          | 0x61 01 70 07        |
//! | 0x70         | Unknown  | Unknown                                                                             |                      |
//! | 0x99         | Unknown  | Unknown                                                                             |                      |
//! | 0xA1         | Unknown  | Unknown                                                                             |                      |
//! | 0xA3         | Unknown  | Unknown                                                                             |                      |
//! | 0xA4         | Unknown  | Unknown                                                                             |                      |
//! | 0xAE         | Unknown  | Unknown                                                                             |                      |
//! | 0xB1         | 4 bytes  | Returns the channel intensities when they are changed.                              | 0xB1 0F 00 00        |
//! | 0xBD         | Unknown  | Unknown                                                                             |                      |
//! | 0xBE         | 6 bytes  | Returns the intensity limits and waveform balance parameters when they are changed. |                      |
//! | 0xC9         | Unknown  | Unknown                                                                             |                      |
//! | 0xD1         | Unknown  | Unknown                                                                             |                      |
//! | 0xE0         | 3 bytes  | Error output message.                                                               | 0xE0 01 00           |
//! | 0xE2         | Unknown  | Unknown                                                                             |                      |
//! | 0xED         | Unknown  | Unknown                                                                             |                      |
//! | 0xF1         | 4 bytes  | Unknown. Might be dupe of 0xB1                                                      | 0xF1 01 00 00        |
//! | 0xF2         | 20 bytes | Unknown. Rcv'd after 0xBF and 0xFF. Contains trigger action config                  | 0xF2 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 |
//! | 0xF3         | 20 bytes | Unknown. Rcv'd after 0xBF                                                           | 0xF3 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 |
//! | 0xF4         | 14 bytes | Unknown. Rcv'd after 0xBF                                                           | 0xF4 00 00 FF FF 00 00 00 00 00 00 00 00 00 |
//!

use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use super::commands::*;

/// ## Notification 0x02 - Pawprint Paired
/// This notification is a response to the 0x01 command. The app issues the 0x01 command when the user searches for accessories. This notification indicates an accessory was successfully paired with the device.
///
/// ### Payload
/// The payload is 4 bytes long. The first byte is the pawprint number, the third byte is the battery level. The second and fourth bytes are unknown, but one of them is a version number.
///
/// | Offset | Size | Type | Description     |
/// |--------|------|------|-----------------|
/// | 0      | 1    | u8   | Pawprint number |
/// | 1      | 1    | u8   | Unknown         |
/// | 2      | 1    | u8   | Battery level   |
/// | 3      | 1    | u8   | Unknown         |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification02PawprintPaired {
    pub pawprint_number: u8,
    pub unknown1: u8,
    pub battery_level: u8,
    pub unknown2: u8,
}

/// ## Notification 0x03 - No Pawprint Found
/// This notification is a response to the 0x01 command. The app issues the 0x01 command when the user searches for accessories. This notification indicates an accessory was not found.
///
/// ### Payload
/// This notification has no payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification03NoPawprintFound {}

/// ## Notification 0x0F - Pawprint Connected
/// This notification is sent when a previously paired pawprint is connects to the coyote.
///
/// ### Payload
/// The payload appears to be identical to the 0x02 notification.
///
/// The payload is 4 bytes long. The first byte is the pawprint number, the third byte is the battery level. The second and fourth bytes are unknown, but one of them is a version number.
/// | Offset | Size | Type | Description     |
/// |--------|------|------|-----------------|
/// | 0      | 1    | u8   | Pawprint number |
/// | 1      | 1    | u8   | Unknown         |
/// | 2      | 1    | u8   | Battery level   |
/// | 3      | 1    | u8   | Unknown         |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification0FPawprintConnected {
    pub pawprint_number: u8,
    pub unknown1: u8,
    pub battery_level: u8,
    pub unknown2: u8,
}

/// ## Notification 0x31 - Trigger Condition Set
/// This notification is a response to the 0x30 command. The app issues the 0x30 command when configuring Trigger Conditions.
///
/// The payload is 19 bytes long and is a copy of the data sent in the 0x30 command.
pub type Notification31TriggerConditionSet = Command30SetTriggerCondition;

/// ## Notification 0x53 - Unknown
/// This notification is sent immediately after subscribing to the notification characteristic (0x150B).
///
/// The payload is 5 bytes long. The 1st byte is 0x00 and the remaining 4 bytes are the last 4 bytes of the BT MAC.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification53Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x61 - Unknown
/// This notification is a response to the 0x60 command. The app issues the 0x60 command when changing a pawprint's shoulder lights or when clearing the trigger params.
///
/// ### Payload
/// The payload is 3 bytes long and is a copy of the data sent in the 0x60 command.
pub type Notification61Unknown = Command60Unknown;

/// ## Notification 0xAE - WaveformsRestored
/// This notification is a response to the 0xAF command. The app issues the 0xAF command when factory restoring waveforms.
///
/// ### Payload
/// This notification has no payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationAEWaveformsRestored {}

/// ## Notification 0xB1 - IntensityChanged
/// This notification is a reply to the 0xB0 command.
///
/// > When the pulse host strength changes, the current strength value will be immediately returned through the B1 message. If the strength change is caused by the B0 command, the sequence number returned in the B1 command will be the same as the sequence number contained in the command that caused the change, otherwise the sequence number is 0.
///
/// ### Payload
/// The payload is 3 bytes long.
///
/// | Offset | Size | Type | Description                    |
/// |--------|------|------|--------------------------------|
/// | 0      | 1    | u8   | Serial number                  |
/// | 1      | 1    | u8   | Current intensity of channel A |
/// | 2      | 1    | u8   | Current intensity of channel B |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationB1IntensityChanged {
    /// 1 byte, 0 ~ 200
    pub serial_number: u8,
    /// 1 byte, 0 ~ 200
    pub current_intensity_a: u8,
    /// 1 byte, 0 ~ 200
    pub current_intensity_b: u8,
}

/// ## Notification 0xBE - LimitsChanged
/// This notification is a reply to [CommandBFSetLimits].
///
/// > The BE message returns the current AB channel strength soft upper limit + AB channel waveform frequency balance parameter + AB channel waveform strength balance parameter of the pulse host after the corresponding setting of BF input.
///
/// ### Payload
/// The payload is 6 bytes long.
///
/// | Offset | Size | Type | Description                          |
/// |--------|------|------|--------------------------------------|
/// | 0      | 1    | u8   | Channel A intensity soft upper limit |
/// | 1      | 1    | u8   | Channel B intensity soft upper limit |
/// | 2      | 1    | u8   | Channel A waveform frequency balance |
/// | 3      | 1    | u8   | Channel B waveform frequency balance |
/// | 4      | 1    | u8   | Channel A waveform intensity balance |
/// | 5      | 1    | u8   | Channel B waveform intensity balance |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationBELimitsChanged {
    /// 2 bytes, 0 ~ 200
    pub limit_a: u8,
    /// 2 bytes, 0 ~ 200
    pub limit_b: u8,
    /// 2 bytes, 0 ~ 255
    pub freq_balance_a: u8,
    /// 2 bytes, 0 ~ 255
    pub freq_balance_b: u8,
    /// 2 bytes, 0 ~ 255
    pub intensity_balance_a: u8,
    /// 2 bytes, 0 ~ 255
    pub intensity_balance_b: u8,
}

/// ## Notification 0xE0 - Error
/// This notification is sent when an error occurs during command processing.
/// 
/// ### Payload
/// The payload is 2 bytes long.
/// 
/// | Offset | Size | Type | Description |
/// |--------|------|------|-------------|
/// | 0      | 1    | u8   | Command     |
/// | 1      | 1    | u8   | Error code  |
///
/// ### Error Codes
/// 
/// | Code | Description                                            |
/// |------|--------------------------------------------------------|
/// | 0x01 | The command type is not understood                     |
/// | 0x02 | The message format is incorrect                        |
/// | 0x03 | This command cannot be executed during accessory gameplay |
/// | 0x04 | The object does not exist (you need to get the gameplay configuration information again) |
/// | 0x05 | Writing to flash failed                                |
/// | 0x06 | Command queue is full                                  |
/// | 0x07 | UNKNOWN                                                |
/// 
/// ### Example
/// ```
/// 0xE0 0x03 0x01
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotificationE0Error {
    /// 1 byte, Command
    pub command: u8,
    /// 1 byte, Error code
    pub error_code: u8,
}

/// ## Notification 0xF1 - Unknown
///
/// This notification is recieved after sending the 0xFF command. The app issues the 0xFF command as part of the initial connection.
///
/// The payload is 3 bytes long, and appears to be identical to the 0xB1 notification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationF1Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xF2 - Unknown
///
/// This notification is recieved after sending the 0xFF command.
///
/// The payload is 19 bytes long. Unknown contents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationF2Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xF3 - Unknown
///
/// This notification is recieved after sending the 0xFF command.
///
/// The payload is 19 bytes long. Unknown contents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationF3Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xF4 - Unknown
///
/// This notification is recieved after sending the 0xFF command.
///
/// The payload is 13 bytes long. Unknown contents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationF4Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Unknown Notification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct UnknownNotification {
    pub notification_id: u8,
    #[deku(read_all)]
    pub data: Vec<u8>,
}
