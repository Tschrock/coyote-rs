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
//! | Notification | Description                                                        |
//! |--------------|--------------------------------------------------------------------|
//! | 0x02         | Pawprint Paired                                                    |
//! | 0x03         | No Pawprint Found                                                  |
//! | 0x04         | Unknown                                                            |
//! | 0x09         | Pawprint Unpaired                                                  |
//! | 0x0C         | Unknown                                                            |
//! | 0x0D         | Unknown                                                            |
//! | 0x0E         | Manual Broadcasting Mode                                           |
//! | 0x0F         | Pawprint Connected                                                 |
//! | 0x10         | Accessory Gameplay Response                                        |
//! | 0x21         | Trigger Action Set                                                 |
//! | 0x29         | Unknown                                                            |
//! | 0x31         | Trigger Condition Set                                              |
//! | 0x41         | Unknown                                                            |
//! | 0x51         | Unknown (Response to 0x50 command)                                 |
//! | 0x53         | Unknown                                                            |
//! | 0x61         | Unknown (Response to 0x60 command)                                 |
//! | 0x70         | Unknown                                                            |
//! | 0x99         | Unknown                                                            |
//! | 0xA1         | Unknown (Response to 0xA0 command - Save waveforms pt1)            |
//! | 0xA3         | Unknown (Response to 0xA2 command - Save waveforms pt2)            |
//! | 0xA4         | Might be the error response for 0xA2?                              |
//! | 0xAE         | Waveforms Restored                                                 |
//! | 0xB1         | Intensity Changed                                                  |
//! | 0xBD         | Unknown                                                            |
//! | 0xBE         | Balance and Limits Changed                                         |
//! | 0xC9         | Unknown                                                            |
//! | 0xD1         | Unknown                                                            |
//! | 0xE0         | Error                                                              |
//! | 0xE2         | Unknown                                                            |
//! | 0xED         | Unknown                                                            |
//! | 0xF1         | Intensity Status                                                   |
//! | 0xF2         | Trigger Action Config                                              |
//! | 0xF3         | Trigger Condition Config                                           |
//! | 0xF4         | Unknown. Rcv'd after 0xBF                                          |
//!

use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

use crate::v3::common::Color;

use super::commands::*;

/// ## Notification 0x02 - Pawprint Paired
/// This notification is a response to the [0x01 - Pair Pawprint] command. It indicates an accessory was successfully paired with the device.
///
/// ### Payload
/// | Offset | Size | Type | Description     |
/// |--------|------|------|-----------------|
/// | 0      | 1    | u8   | Pawprint number |
/// | 1      | 1    | u8   | Unknown         |
/// | 2      | 1    | u8   | Battery level   |
/// | 3      | 1    | u8   | Unknown         |
/// 
/// [0x01 - Pair Pawprint]: crate::v3::commands::Command01PairPawprint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification02PawprintPaired {
    pub pawprint_number: u8,
    pub unknown1: u8,
    pub battery_level: u8,
    pub unknown2: u8,
}

/// ## Notification 0x03 - No Pawprint Found
/// This notification is a response to the [0x01 - Pair Pawprint] command. It indicates an accessory was not found.
///
/// ### Payload
/// This notification has no payload.
/// 
/// [0x01 - Pair Pawprint]: crate::v3::commands::Command01PairPawprint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification03NoPawprintFound {}

/// ## Notification 0x04 - Unknown
/// Unknown
/// 
/// ### Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification04Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x09 - Pawprint Unpaired
/// Response to the [0x08 - Unpair Pawprint] command.
/// 
/// ### Payload
/// Same as [0x08 - Unpair Pawprint].
/// 
/// [0x08 - Unpair Pawprint]: crate::v3::commands::Command08UnpairPawprint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification09PawprintUnpaired {
    pub pawprint_number: u8,
}

/// ## Notification 0x0C - Unknown
/// Unknown
/// 
/// ### Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification0CUnknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x0D - Unknown
/// Response to the [0x0D - Unknown] command.
/// 
/// ### Payload
/// Unknown
/// 
/// ## Examples
/// - 00000000000000000000000000000000 - Before pairing - response to 0d command
/// - fffffffe000000000000000000000002 - When pairing - response to 0dffffffffffffffffffffffffffffffff command
/// - 7e769b9ffabf00000000000000000502 - on connection after pairing - response to 0d command
/// - 7e769b9ffabf00000000000000000400 - When unpairing - response to 0d00000000000000000000000000000000 command
/// - 7e769b9ffabf00000000000000000400 - on connection after unpairing - response to 0d command
/// 
/// - 7e769b9ffabf00000000000000000400 - Before pairing
/// - fffffffefabf00000000000000000402 - After 0dffffffffffffffffffffffffffffffff
/// - 0c responds with 7e769b9ffabf00000000000000000502
/// - Data: 7e769b9ffabf00000000000000000502 - on connection after pairing - response to 0d command
/// 
/// [0x0D - Unknown]: crate::v3::commands::Command0DUnknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification0DUnknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x0E - Manual Broadcasting Mode
/// Response to the [0x0E - Manual Broadcasting Mode] command.
/// 
/// > Note: these being 0x0A and 0x0B makes me think it's a bitfield, but I haven't seen any other values yet.
/// 
/// ### Payload
/// - `0x0A` - Manual broadcasting mode enabled
/// - `0x0B` - Manual broadcasting mode disabled
/// 
/// [0x0E - Manual Broadcasting Mode]: crate::v3::commands::Command0EManualBroadcastingMode
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Notification0EManualBroadcastingMode {
    Enabled = 0x0A,
    Disabled = 0x0B,
}

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

/// ## Notification 0x10 - Accessory Gameplay Response
/// Sent in response to the [0x11 - Start Accessory Gameplay] and [0x12 - Stop Accessory Gameplay] commands.
///
/// ### Payload
/// Unknown. The first byte is 0x03 when starting gameplay and 0x01 when stopping gameplay.
///
/// ### Examples
/// - 03bdd8 - Starting response
/// - 010000 - Stopping response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification10AccessoryGameplayResponse {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x21 - Trigger Action Set
/// Response to the [0x20 - Set Trigger Action] command.
///
/// ### Payload
/// A copy of the data sent in the 0x20 command. See [Command20SetTriggerAction] for details.
///
pub type Notification21TriggerActionSet = Command20SetTriggerAction;

/// ## Notification 0x29 - Unknown
/// Unknown
/// 
/// ### Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification29Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x31 - Trigger Condition Set
/// Response to the [0x30 - Set Trigger Condition] command.
///
/// ### Payload
/// A copy of the data sent in the 0x30 command. See [Command30SetTriggerCondition] for details.
pub type Notification31TriggerConditionSet = Command30SetTriggerCondition;

/// ## Notification 0x41 - Unknown
/// Unknown
/// 
/// ### Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification41Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x51 - Unknown
/// Response to the [0x50 - Unknown] command. It is also sent when the battery level changes.
///
/// ### Payload
/// | Offset | Size | Type | Description     |
/// |--------|------|------|-----------------|
/// | 0      | 1    | u8   | Indicator Color |
/// | 1      | 1    | u8   | Unknown         |
/// | 2      | 1    | u8   | Battery level   |
/// 
/// See [Color] for possible indicator color values.
/// 
/// ### Examples
/// - `03 10 41` - Indicator color 3, unknown, battery 65%
/// 
/// [0x50 - Unknown]: crate::v3::commands::Command50Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification51Unknown {
    pub indicator_color: Color,
    pub unknown: u8,
    pub battery_level: u8,
}

/// ## Notification 0x53 - Unknown
/// This notification is sent immediately after subscribing to the notification characteristic (0x150B).
///
/// ### Payload
/// | Offset | Size | Type | Description       |
/// |--------|------|------|-------------------|
/// | 0      | 1    | u8   | Unknown           |
/// | 1      | 4    | u8   | Last 4 of BT MAC  |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification53Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x61 - Unknown
/// Response to the [0x60 - Accessory Command] command.
///
/// ### Payload
/// A copy of the data sent in the [0x60 - Accessory Command] command.
/// 
/// [0x60 - Accessory Command]: crate::v3::commands::Command60AccessoryCommand
pub type Notification61Unknown = Command60AccessoryCommand;

/// ## Notification 0x70 - Angle Threshold Detected
/// Sent once angle thresholds have been detected. Detection can be started with the angle detection [0x60 - Accessory Command].
/// 
/// ### Payload
/// | Offset | Size | Type | Description                        |
/// |--------|------|------|------------------------------------|
/// | 0      | 1    | u8   | Pawprint number                    |
/// | 1      | 1    | u8   | 0x61 (response to 0x60 subcommand) |
/// | 2      | 2    | i16  | X minimum angle in degrees         |
/// | 4      | 2    | i16  | X maximum angle in degrees         |
/// | 6      | 2    | i16  | Y minimum angle in degrees         |
/// | 8      | 2    | i16  | Y maximum angle in degrees         |
/// | 10     | 2    | i16  | Z minimum angle in degrees         |
/// | 12     | 2    | i16  | Z maximum angle in degrees         |
///
/// ### Examples
/// ```
/// 70 01 61 fff5 0040 ffd6 0014 0065 007d
/// ```
/// - Pawprint number: 1
/// - X: -11 to 64
/// - Y: -42 to 20
/// - Z: 101 to 125
/// 
/// > Note: The sliders in the app go by 2s so only even numbers show up there.
/// 
/// [0x60 - Accessory Command]: crate::v3::commands::Command60AccessoryCommand
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification70Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x99 - Unknown
/// Unknown
/// 
/// ## Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification99Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xA1 - Unknown
///
/// This notification is a response to the 0xA0 command.
///
/// ### Payload
///
/// | Offset | Size | Type | Description     |
/// |--------|------|------|-----------------|
/// | 0      | 1    | u8   | Slot number     |
/// | 1      | 1    | u8   | Unknown         |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationA1Unknown {
    pub slot: u8,
    pub unknown: u8,
}

/// ## Notification 0xA3 - Unknown
///
/// This notification is a response to the 0xA2 command.
///
/// ### Payload
///
/// | Offset | Size | Type | Description     |
/// |--------|------|------|-----------------|
/// | 0      | 1    | u8   | Slot number     |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationA3Unknown {
    pub slot: u8,
}

/// ## Notification 0xA4 - Unknown
/// Unknown
/// 
/// ## Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationA4Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xAE - Waveforms Restored
/// Response to the [0xAF - Restore Waveforms] command.
///
/// ### Payload
/// This notification has no payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationAEWaveformsRestored {}

/// ## Notification 0xB1 - Intensity Changed
/// A reply to the 0xB0 command. Only sent if the b0 command changed the intensity, or if a dial has caused the intensity to change. Note the dials don't directly trigger this, it's only sent when using the b0 command.
///
/// > When the pulse host strength changes, the current strength value will be immediately returned through the B1 message. If the strength change is caused by the B0 command, the sequence number returned in the B1 command will be the same as the sequence number contained in the command that caused the change, otherwise the sequence number is 0.
///
/// ### Payload
/// The payload is 3 bytes long.
///
/// | Offset | Size | Type | Description                    |
/// |--------|------|------|--------------------------------|
/// | 0      | 1    | u8   | Sequence number                |
/// | 1      | 1    | u8   | Current intensity of channel A |
/// | 2      | 1    | u8   | Current intensity of channel B |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationB1IntensityChanged {
    /// 1 byte, 0 ~ 200
    pub sequence_number: u8,
    /// 1 byte, 0 ~ 200
    pub current_intensity_a: u8,
    /// 1 byte, 0 ~ 200
    pub current_intensity_b: u8,
}

/// ## Notification 0xBD - Unknown
/// Unknown
/// 
/// ## Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationBDUnknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xBE - Limits Changed
/// A reply to [CommandBFSetLimits].
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
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

/// ## Notification 0xC9 - Unknown
/// Unknown. Old strength header?
/// 
/// ## Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationC9Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xD1 - Unknown
/// Unknown
/// 
/// ## Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationD1Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationE0Error {
    /// 1 byte, Command
    pub command: u8,
    /// 1 byte, Error code
    pub error_code: u8,
}

/// ## Notification 0xE2 - Unknown
/// Unknown
/// 
/// ## Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationE2Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xED - Unknown
/// Unknown
/// 
/// ## Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationEDUnknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xF1 - Intensity Status
/// Received after sending the 0xFF command.
///
/// ### Payload
/// The payload is 3 bytes long, and appears to be identical to the 0xB1 notification.
///
/// | Offset | Size | Type | Description                    |
/// |--------|------|------|--------------------------------|
/// | 0      | 1    | u8   | Sequence number                |
/// | 1      | 1    | u8   | Current intensity of channel A |
/// | 2      | 1    | u8   | Current intensity of channel B |
pub type NotificationF1Unknown = NotificationB1IntensityChanged;

/// ## Notification 0xF2 - Trigger Action Config
/// Received after sending the 0xFF command. It is repeated for each configured Trigger Action.
///
/// ### Payload
/// The payload is one of the currently configured trigger actions. See [Command20SetTriggerAction] for details.
pub type NotificationF2TriggerActionConfig = Command20SetTriggerAction;

/// ## Notification 0xF3 - Trigger Condition Config
/// Received after sending the 0xFF command. It is repeated for each configured Trigger Condition.
///
/// ### Payload
/// The payload is one of the currently configured trigger conditions. See [Command30SetTriggerCondition] for details.
pub type NotificationF3TriggerConditionConfig = Command30SetTriggerCondition;

/// ## Notification 0xF4 - Unknown
/// Received after sending the 0xFF command.
///
/// ### Payload
/// Unknown
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Notification {
    #[deku(id = "0x02")]
    PawprintPaired(Notification02PawprintPaired),
    #[deku(id = "0x03")]
    NoPawprintFound(Notification03NoPawprintFound),
    #[deku(id = "0x04")]
    Unknown04(Notification04Unknown),
    #[deku(id = "0x09")]
    PawprintUnpaired(Notification09PawprintUnpaired),
    #[deku(id = "0x0C")]
    Unknown0C(Notification0CUnknown),
    #[deku(id = "0x0D")]
    Unknown0D(Notification0DUnknown),
    #[deku(id = "0x0E")]
    ManualBroadcastingMode(Notification0EManualBroadcastingMode),
    #[deku(id = "0x0F")]
    PawprintConnected(Notification0FPawprintConnected),
    #[deku(id = "0x10")]
    AccessoryGameplayResponse(Notification10AccessoryGameplayResponse),
    #[deku(id = "0x21")]
    TriggerActionSet(Notification21TriggerActionSet),
    #[deku(id = "0x29")]
    Unknown29(Notification29Unknown),
    #[deku(id = "0x31")]
    TriggerConditionSet(Notification31TriggerConditionSet),
    #[deku(id = "0x41")]
    Unknown41(Notification41Unknown),
    #[deku(id = "0x51")]
    Unknown51(Notification51Unknown),
    #[deku(id = "0x53")]
    Unknown53(Notification53Unknown),
    #[deku(id = "0x61")]
    Unknown61(Notification61Unknown),
    #[deku(id = "0x70")]
    Unknown70(Notification70Unknown),
    #[deku(id = "0x99")]
    Unknown99(Notification99Unknown),
    #[deku(id = "0xA1")]
    UnknownA1(NotificationA1Unknown),
    #[deku(id = "0xA3")]
    UnknownA3(NotificationA3Unknown),
    #[deku(id = "0xA4")]
    UnknownA4(NotificationA4Unknown),
    #[deku(id = "0xAE")]
    WaveformsRestored(NotificationAEWaveformsRestored),
    #[deku(id = "0xB1")]
    IntensityChanged(NotificationB1IntensityChanged),
    #[deku(id = "0xBD")]
    UnknownBD(NotificationBDUnknown),
    #[deku(id = "0xBE")]
    LimitsChanged(NotificationBELimitsChanged),
    #[deku(id = "0xC9")]
    UnknownC9(NotificationC9Unknown),
    #[deku(id = "0xD1")]
    UnknownD1(NotificationD1Unknown),
    #[deku(id = "0xE0")]
    CommandError(NotificationE0Error),
    #[deku(id = "0xE2")]
    UnknownE2(NotificationE2Unknown),
    #[deku(id = "0xED")]
    UnknownED(NotificationEDUnknown),
    #[deku(id = "0xF1")]
    IntensityStatus(NotificationF1Unknown),
    #[deku(id = "0xF2")]
    TriggerActionConfig(NotificationF2TriggerActionConfig),
    #[deku(id = "0xF3")]
    TriggerConditionConfig(NotificationF3TriggerConditionConfig),
    #[deku(id = "0xF4")]
    UnknownF4(NotificationF4Unknown),
    #[deku(id_pat = "_")]
    UnknownNotification(u8, UnknownCommand),
}
