//! Pulse host commands
//!
//! Commands are formatted as a single byte identifier followed by a variable length payload.
//!
//! ```
//! +-----+------+------+------+------>
//! | CMD | DATA                  ... |
//! +-----+------+------+------+------>
//! ```
//!
//! ## Device Commands
//!
//! | Command | Description                                               |
//! |---------|-----------------------------------------------------------|
//! | [0x01]  | Pair Pawprint                                             |
//! | [0x08]  | Unpair Pawprint                                           |
//! | [0x0E]  | Manual Broadcasting Mode                                  |
//! | [0x0C]  | Unknown                                                   |
//! | [0x0D]  | Unknown                                                   |
//! | [0x11]  | Start Accessory Gameplay                                  |
//! | [0x12]  | Stop Accessory Gameplay                                   |
//! | [0x20]  | Set Trigger Action                                        |
//! | [0x28]  | Delete Trigger Action                                     |
//! | [0x30]  | Set Trigger Condition                                     |
//! | [0x40]  | Update Accessory Settings                                 |
//! | [0x50]  | Unknown - Device Indicator Color                          |
//! | [0x60]  | Set color or clear trigger parameters.                    |
//! | [0xA0]  | Unknown                                                   |
//! | [0xA2]  | Unknown                                                   |
//! | [0xAF]  | Restore Waveforms                                         |
//! | [0xB0]  | Set Intensity and Waveform Data                           |
//! | [0xBC]  | Unknown                                                   |
//! | [0xBF]  | Set Balance and Limits                                    |
//! | [0xFE]  | Clear Accessory Data                                      |
//! | [0xFF]  | Get All Accessory Data                                    |
//!
//! [0x01]: crate::v3::commands::Command01PairPawprint
//! [0x08]: crate::v3::commands::Command08UnpairPawprint
//! [0x0E]: crate::v3::commands::Command0EManualBroadcastingMode
//! [0x0C]: crate::v3::commands::Command0CUnknown
//! [0x0D]: crate::v3::commands::Command0DUnknown
//! [0x11]: crate::v3::commands::Command11StartAccessoryGameplay
//! [0x12]: crate::v3::commands::Command12StopAccessoryGameplay
//! [0x20]: crate::v3::commands::Command20SetTriggerAction
//! [0x28]: crate::v3::commands::Command28DeleteTriggerAction
//! [0x30]: crate::v3::commands::Command30SetTriggerCondition
//! [0x40]: crate::v3::commands::Command40UpdateAccessorySettings
//! [0x50]: crate::v3::commands::Command50Unknown
//! [0x60]: crate::v3::commands::Command60AccessoryCommand
//! [0xA0]: crate::v3::commands::CommandA0Unknown
//! [0xA2]: crate::v3::commands::CommandA2Unknown
//! [0xAF]: crate::v3::commands::CommandAFRestoreWaveforms
//! [0xB0]: crate::v3::commands::CommandB0SetIntensity
//! [0xBC]: crate::v3::commands::CommandBCUnknown
//! [0xBF]: crate::v3::commands::CommandBFSetLimits
//! [0xFE]: crate::v3::commands::CommandFEClearAccessoryData
//! [0xFF]: crate::v3::commands::CommandFFGetAllAccessoryData
//!

use deku::{bitvec::BitVec, ctx::Endian, prelude::*};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};

use crate::v3::common::{Color, Waveform};

/// ## Command 0x01 - Pair pawprint
/// Tells the device to search for and pair with a nearby pawprint.
///
/// Once the coyote finds and pairs with the pawprint, it will respond with an [0x02 - Pawprint Paired] notification.
/// If a pawprint is not found within 10 seconds, the coyote will stop searching and respond with an [0x03 - No Pawprint Found] notification.
///
/// ### Payload
/// This command has no payload.
///
/// ### Responses
/// - [0x02 - Pawprint Paired]: Pawprint successfully paired.
/// - [0x03 - No Pawprint Found]: Pawprint pairing timed out.
///
/// [0x02 - Pawprint Paired]: crate::v3::notifications::Notification02PawprintPaired
/// [0x03 - No Pawprint Found]: crate::v3::notifications::Notification03NoPawprintFound
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command01PairPawprint {}

/// ## Command 0x08 - Unpair Pawprint
/// Tells the device to unpair with a pawprint.
///
/// ### Payload
/// The pawprint number to unpair. Each pawprint is assigned a number based on the order they were paired with the device, starting at 1.
///
/// ### Examples
/// - `08 01` - Unpair pawprint 1
/// - `08 02` - Unpair pawprint 2
///
/// ### Response
/// The device will respond with a [0x09 - Pawprint Unpaired] notification.
///
/// [0x09 - Pawprint Unpaired]: crate::v3::notifications::Notification09PawprintUnpaired
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command08UnpairPawprint {
    pub pawprint_number: u8,
}

/// ## Command 0x0E - Manual Broadcasting Mode
/// Gets or sets the Manual Broadcast mode. When in manual broadcast mode, the device will not be discoverable over Bluetooth unless the user manually flips both dials down and continues holding them while connecting.
///
/// ### Payload
/// One of the following values:
/// - `0x00` - Get status
/// - `0x01` - Turn on
/// - `0x02` - Turn off
///
/// ### Response
/// The device will respond with a [0x0E - Manual Broadcasting Mode] notification containing the configured mode.
///
/// [0x0E - Manual Broadcasting Mode]: crate::v3::notifications::Notification0EManualBroadcastingMode
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Command0EManualBroadcastingMode {
    GetStatus = 0x00,
    TurnOn = 0x01,
    TurnOff = 0x02,
}

/// ## Command 0x0C - Unknown
/// Sent after [0x0D - Unknown] command when pairing (but not when unpairing).
///
/// ### Payload
/// Unknown
///
/// ### Example
/// ```
/// 0c00000001
/// ```
///
/// ### Response
/// The device will respond with a [0x0C - Unknown] notification.
///
/// [0x0C - Unknown]: crate::v3::notifications::Notification0CUnknown
/// [0x0D - Unknown]: crate::v3::commands::Command0DUnknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command0CUnknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Command 0x0D - Unknown
/// Sent when connecting, pairing, and unpairing. When pairing is enabled, the device will pair with the phone and will respond with an error notification (0xe0ff07) to the starting 0xFF command if the correct phone is not paired.
///
/// ### Payload
/// When connecting, sent with no payload. (0x0d)
///
/// When pairing, sent with 16 full bytes (0x0dffffffffffffffffffffffffffffffff)
///
/// When unpairing, sent with 16 empty bytes (0x0d00000000000000000000000000000000)
///
/// ### Response
/// The device will respond with a [0x0D - Unknown] notification.
///
/// [0x0D - Unknown]: crate::v3::notifications::Notification0DUnknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command0DUnknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Command 0x11 - Start Accessory Gameplay
/// Starts accessory gameplay.
///
/// ### Payload
/// This command has no payload.
///
/// ## Response
/// The device will respond with a [0x10 - Accessory Gameplay Response] notification. You may also receive [0x0F - Pawprint Connected] notifications for connected pawprints.
///
/// [0x10 - Accessory Gameplay Response]: crate::v3::notifications::Notification10AccessoryGameplayResponse
/// [0x0F - Pawprint Connected]: crate::v3::notifications::Notification0FPawprintConnected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command11StartAccessoryGameplay {}

/// ## Command 0x12 - Stop Accessory Gameplay
/// Stops accessory gameplay.
///
/// ### Payload
/// This command has no payload.
///
/// ## Response
/// The device will respond with a [0x10 - Accessory Gameplay Response] notification.
///
/// [0x10 - Accessory Gameplay Response]: crate::v3::notifications::Notification10AccessoryGameplayResponse
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command12StopAccessoryGameplay {}

/// ## Command 0x20 - Set Trigger Action
/// Sent when the user configures a trigger action in the app.
///
/// ### Payload
/// The payload is 19 bytes long.
///
/// | Offset | Type | Default | Description                                             |
/// |--------|------|---------|---------------------------------------------------------|
/// | 0x00   | u8   | 0x01    | Trigger ID.                                             |
/// | 0x01   | u8   | 0x10    | Bit map containing some of the boolean trigger options. |
/// | 0x02   | u8   | 0x00    | Temporary output waveform for channel A.                |
/// | 0x03   | u8   | 0x00    | Temporary output waveform for channel B.                |
/// | 0x04   | u8   | 0x00    | Temporary intensity change mode for channel A.          |
/// | 0x05   | i8   | 0x00    | Temporary intensity change lower bound for channel A.   |
/// | 0x06   | i8   | 0x00    | Temporary intensity change upper bound for channel A.   |
/// | 0x07   | u8   | 0x00    | Temporary intensity change mode for channel B.          |
/// | 0x08   | i8   | 0x00    | Temporary intensity change lower bound for channel B.   |
/// | 0x09   | i8   | 0x00    | Temporary intensity change upper bound for channel B.   |
/// | 0x0A   | i8   | 0x00    | Permanent intensity change lower bound for channel A.   |
/// | 0x0B   | i8   | 0x00    | Permanent intensity change upper bound for channel A.   |
/// | 0x0C   | i8   | 0x00    | Permanent intensity change lower bound for channel B.   |
/// | 0x0D   | i8   | 0x00    | Permanent intensity change upper bound for channel B.   |
/// | 0x0E   | i16  | 0x00    | Adjust remaining session time lower bound.              |
/// | 0x10   | i16  | 0x00    | Adjust remaining session time upper bound.              |
/// | 0x12   | u8   | 0x00    | Temporary trigger delay mode.                           |
///
/// #### Options Bit Map (offset 0x01)
/// Each bit corresponds to a boolean option. From left to right (MSB first), they are:
/// - bit 0: Not used
/// - bit 1: Not used
/// - bit 2: Not used
/// - bit 3: Allow repeated triggering (inverted, so 0 = allow and 1 = disallow)
/// - bit 4: Not used
/// - bit 5: Not used
/// - bit 6: Stop play session immediately
/// - bit 7: Skip delay and start immediately
///
/// #### Temporary Output Waveform (offsets 0x02 and 0x03)
/// The value is an unsigned 8-bit integer that corresponds to a waveform saved in device memory.
///
/// See [Waveform] for available waveforms.
///
/// The app allows users to upload custom waveforms to the 8 "Built-in mode" slots. These slots are are used when operating the device without a connected phone. When disconnected, the shoulder dials can be pressed to toggle between 4 different color modes. These modes using the following mapping:
/// - Yellow, Channel A: Built-in mode1
/// - Yellow, Channel B: Built-in mode2
/// - Blue, Channel A: Built-in mode3
/// - Blue, Channel B: Built-in mode4
/// - Cyan, Channel A: Built-in mode5
/// - Cyan, Channel B: Built-in mode6
/// - Purple, Channel A: Built-in mode7
/// - Purple, Channel B: Built-in mode8
///
/// #### Temporary Intensity Change Mode (offsets 0x04 and 0x07)
/// The intensity change mode is a single byte. It packs together both the mode and (in Gradient mode) the gradient value.
/// - 0x01: Fixed
/// - 0x02: Random
/// - 0x03: Parameter Mapping
/// - 0x04-0xFF: Gradient, with the gradient value being (x - 3)
///
/// #### Temporary Intensity Change Lower/Upper Bound (offsets 0x05, 0x06, 0x08, and 0x09)
/// These are signed 8-bit integers. The lower bound must be less than or equal to the upper bound.
///
/// - In `Fixed` mode, both the lower and upper bounds are the same value.
/// - In `Random` mode, the value uses is randomized between the lower and upper bounds.
/// - In `Parameter Mapping` mode, the value is mapped between the lower and upper bounds based on the trigger parameter.
/// - In `Gradient` mode, the value gradually changes from the lower to the upper bound, changing by 1 every (gradient value) seconds.
///
/// TODO: What happens if the mode is `Fixed` but the upper and lower bounds are different?
///
/// #### Permanent Intensity Change Lower/Upper Bounds (offsets 0x0A-0x0D)
/// These are signed 8-bit integers. The lower bound must be less than or equal to the upper bound.
///
/// When the values are the same, the intensity change operates in `Fixed` mode. When different, the intensity change operates in `Random` mode.
///
/// ### Example
/// ```hex
/// 02 03 07 0d 28 e3 13 02 f0 ff f9 03 fa 05 f7 cc f7 cc 07
/// ```
///
/// | Offset | Description                   | Raw Value | Value                       |
/// |--------|-------------------------------|-----------|-----------------------------|
/// | 0      | Trigger ID                    | 0x02      | 2                           |
/// | 1      | Options                       | 0x03      | 0b00000011                  |
/// | 2      | Temp waveform ch A            | 0x07      | Built-in mode7              |
/// | 3      | Temp waveform ch B            | 0x0D      | Preset Punishment Waveform3 |
/// | 4      | Temp intensity change A mode  | 0x28      | Gradient(37)                |
/// | 5      | Temp intensity change A lower | 0xE3      | -29                         |
/// | 6      | Temp intensity change A upper | 0x13      | 19                          |
/// | 7      | Temp intensity change B mode  | 0x02      | Random                      |
/// | 8      | Temp intensity change B lower | 0xF0      | -16                         |
/// | 9      | Temp intensity change B upper | 0xFF      | -1                          |
/// | 10     | Perm intensity change A lower | 0xF9      | -7                          |
/// | 11     | Perm intensity change A upper | 0x03      | 3                           |
/// | 12     | Perm intensity change B lower | 0xFA      | -6                          |
/// | 13     | Perm intensity change B upper | 0x05      | 5                           |
/// | 14     | Adjust time lower bound       | 0xF7CC    | -35 minutes                 |
/// | 16     | Adjust time upper bound       | 0xF7CC    | -35 minutes                 |
/// | 18     | Temporary trigger mode        | 0x07      | Delay(7)                    |
///
/// ### Response
/// TODO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command20SetTriggerAction {
    pub trigger_id: u8,
    #[deku(
        pad_bits_before = "3",
        reader = "read_bit_inverted(deku::reader)",
        writer = "write_bit_inverted(deku::writer, &self.allow_repeated_triggering)"
    )]
    pub allow_repeated_triggering: bool,
    #[deku(pad_bits_before = "2", bits = 1)]
    pub stop_play_session_immediately: bool,
    #[deku(bits = 1)]
    pub skip_delay_and_start_immediately: bool,
    pub temporary_output_waveform_channel_a: Waveform,
    pub temporary_output_waveform_channel_b: Waveform,
    pub temporary_intensity_change_channel_a_mode: TemporaryIntensityChangeMode,
    pub temporary_intensity_change_channel_a_lower_bound: i8,
    pub temporary_intensity_change_channel_a_upper_bound: i8,
    pub temporary_intensity_change_channel_b_mode: TemporaryIntensityChangeMode,
    pub temporary_intensity_change_channel_b_lower_bound: i8,
    pub temporary_intensity_change_channel_b_upper_bound: i8,
    pub permanent_intensity_change_channel_a_lower_bound: i8,
    pub permanent_intensity_change_channel_a_upper_bound: i8,
    pub permanent_intensity_change_channel_b_lower_bound: i8,
    pub permanent_intensity_change_channel_b_upper_bound: i8,
    #[deku(endian = "big")]
    pub adjust_remaining_session_time_lower_bound: i16,
    #[deku(endian = "big")]
    pub adjust_remaining_session_time_upper_bound: i16,
    pub temporary_trigger_mode: TemporaryTriggerMode,
}

fn read_bit_inverted<R: Read + Seek>(reader: &mut Reader<R>) -> Result<bool, DekuError> {
    match reader.read_bits(1, deku::ctx::Order::Msb0)? {
        Some(bits) => Ok(!bits[0]),
        None => return Err(DekuError::Incomplete(NeedSize::new(1))),
    }
}

fn write_bit_inverted<W: Write + Seek>(
    writer: &mut Writer<W>,
    value: &bool,
) -> Result<(), DekuError> {
    let bit = !value;
    writer.write_bits(&BitVec::from_iter(std::iter::once(bit)))?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemporaryIntensityChangeMode {
    Fixed,
    Random,
    ParameterMapping,
    Gradient(u8),
}

impl<'a> DekuReader<'a> for TemporaryIntensityChangeMode {
    fn from_reader_with_ctx<R: Read + Seek>(
        reader: &mut Reader<R>,
        _: (),
    ) -> Result<Self, DekuError> {
        u8::from_reader_with_ctx(reader, Endian::default())?;
        let id = u8::from_reader_with_ctx(reader, ())?;
        match id {
            0x01 => Ok(TemporaryIntensityChangeMode::Fixed),
            0x02 => Ok(TemporaryIntensityChangeMode::Random),
            0x03 => Ok(TemporaryIntensityChangeMode::ParameterMapping),
            _ => Ok(TemporaryIntensityChangeMode::Gradient(id - 3)),
        }
    }
}

impl DekuWriter<()> for TemporaryIntensityChangeMode {
    fn to_writer<W: Write + Seek>(&self, writer: &mut Writer<W>, _: ()) -> Result<(), DekuError> {
        match self {
            TemporaryIntensityChangeMode::Fixed => 0x01,
            TemporaryIntensityChangeMode::Random => 0x02,
            TemporaryIntensityChangeMode::ParameterMapping => 0x03,
            TemporaryIntensityChangeMode::Gradient(x) => x + 3,
        }
        .to_writer(writer, ())?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum TemporaryTriggerMode {
    #[deku(id = "0x00")]
    Sync,
    #[deku(id_pat = "_")]
    Delay(u8),
}

/// ## Command 0x28 - Delete trigger action
/// Deletes a configured trigger action.
///
/// ### Payload
/// The payload is 1 byte long, containing the trigger ID to delete.
///
/// ### Example
/// - `28 02` - Delete trigger action 2
///
/// ### Response
/// TODO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command28DeleteTriggerAction {
    pub trigger_id: u8,
}

/// ## Command 0x30 - Set trigger condition
/// Configures a trigger condition
/// 
/// ### Payload
/// | Offset | Type | Description                     |
/// |--------|------|---------------------------------|
/// | 0      | u8   | Pawprint number                 |
/// | 1      | u8   | Trigger type                    |
/// | 2      | ...  | Trigger parameters              |
/// 
/// Trigger types:
/// - [0x01 - Press/Release](PressReleaseTrigger)
/// - [0x02 - Acceleration/Angle](AccelerationAngleTrigger)
/// - [0x03 - Random Reaction](RandomReactionTrigger)
/// - [0x04 - Random Probability](RandomProbabilityTrigger)
/// - [0x0F - ExternalVoltageInput](ExternalVoltageInputTrigger)
/// 
/// ### Response
/// The device will respond with a [0x31 - Trigger Condition Set] notification.
/// 
/// [0x31 - Trigger Condition Set]: crate::v3::notifications::Notification31TriggerConditionSet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command30SetTriggerCondition {
    pub pawprint_number: u8,
    pub trigger: TriggerCondition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum TriggerCondition {
    #[deku(id = "0x01")]
    PressRelease(PressReleaseTrigger),
    #[deku(id = "0x02")]
    AccelerationAngle(AccelerationAngleTrigger),
    #[deku(id = "0x03")]
    RandomReaction(RandomReactionTrigger),
    #[deku(id = "0x04")]
    RandomProbability(RandomProbabilityTrigger),
    #[deku(id = "0x0F")]
    ExternalVoltageInput(ExternalVoltageInputTrigger),
}

/// ##  Trigger type 0x01 - Press/Release
/// The Press/Release trigger is activated when the user presses or releases the button on a pawprint.
/// 
/// ### Parameter Data
/// | Offset | Type | Description                                |
/// |--------|------|--------------------------------------------|
/// | 0      | u8   | Trigger Action number                      |
/// | 1      | u8   | "Press" = 0, "Release" = 1                 |
/// | 2      | u8   | Param increase rate (slider val)           |
/// | 3      | u8   | Param decrease                             |
/// | 4      | u8   | Param decrease rate (slider val)           |
/// | 5      | u8   | Param increase                             |
/// | 6-13   | ...  | Padding (0x00)                             |
/// 
/// TODO: document slider vals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct PressReleaseTrigger {
    pub trigger_action_number: u8,
    pub is_press: bool,
    pub param_increase_rate: u8,
    pub param_decrease: u8,
    pub param_decrease_rate: u8,
    #[deku(pad_bytes_after = "8")]
    pub param_increase: u8,
}

/// ## Trigger type 0x02 - Acceleration/Angle
/// 
/// ### Parameter Data
/// | Offset | Type | Description                                  |
/// |--------|------|----------------------------------------------|
/// | 0      | u8   | Unknown - Always 0x00                        |
/// | 1      | u8   | Unused (0x00)                                |
/// | 2      | u8   | Unknown - Always 0x00                        |
/// | 3      | u8   | Trigger Action number                        |
/// | 4      | u8   | Acceleration Mode                            |
/// | 5-15   | ...  | Acceleration parameters                      |
/// | 11     | u8   | Parameter mapping max                        |
/// | 12     | u8   | Trigger debounce time (in 0.1s increments)   |
/// | 13     | u8   | Untrigger debounce time (in 0.1s increments) |
/// 
/// Acceleration modes:
/// - [0x00 - Mode 1: Overall Acceleration](OverallAccelerationMode)
/// - [0x02 - Mode 2: Angle Detection](AngleDetectionMode)
/// 
/// Debounce times are in tenths of seconds. So for example, a value of 10 (0x0A) corresponds to 1 second, and a value of 255 (0xFF) corresponds to 25.5 seconds. The maximum time the app can set is 10.0 seconds (100 = 0x64). It's not known if the device accepts higher values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct AccelerationAngleTrigger {
    #[deku(pad_bytes_after = "1")]
    pub unknown1: u8,
    pub unknown2: u8,
    pub trigger_action_number: u8,
    pub acceleration_mode: AccelerationMode,
    pub parameter_mapping_max: u8,
    pub trigger_debounce_time: u8,
    pub untrigger_debounce_time: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum AccelerationMode {
    #[deku(id = "0x01")]
    OverallAcceleration(OverallAccelerationMode),
    #[deku(id = "0x02")]
    AngleDetection(AngleDetectionMode),
}

/// ### Acceleration Mode 1 - Overall Acceleration Mode
/// | Offset | Type | Description                                  |
/// |--------|------|----------------------------------------------|
/// | 0      | u8   | Less than = 0x00, Greater than = 0x01        |
/// | 1      | u16  | Acceleration threshold                       |
/// | 3-5    | ...  | Padding (0x00)                               |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct OverallAccelerationMode {
    pub is_greater_than: bool,
    #[deku(endian = "big", pad_bytes_after = "3")]
    pub acceleration_threshold: u16,
}

/// ### Acceleration Mode 2 - Angle Detection Mode
/// 
/// ### Parameter Data
/// | Offset | Type | Description                                  |
/// |--------|------|----------------------------------------------|
/// | 0      | i8   | X-axis minimum angle (degrees / 2)           |
/// | 1      | i8   | X-axis maximum angle (degrees / 2)           |
/// | 2      | i8   | Y-axis minimum angle (degrees / 2)           |
/// | 3      | i8   | Y-axis maximum angle (degrees / 2)           |
/// | 4      | i8   | Z-axis minimum angle (degrees / 2)           |
/// | 5      | i8   | Z-axis maximum angle (degrees / 2)           |
/// 
/// All angles are in units of `degrees / 2`. So for example, a value of 90 (0x5A) corresponds to 180 degrees. Additionally, a value of [i8::MIN] (-128 = 0x80) means no negative angle limit, and a value of [i8::MAX] (127 = 0x7F) means no positive angle limit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct AngleDetectionMode {
    pub x_min: i8,
    pub x_max: i8,
    pub y_min: i8,
    pub y_max: i8,
    pub z_min: i8,
    pub z_max: i8,
}

/// ### Trigger type 0x03 - Random Reaction
/// | Offset | Type | Description                          |
/// |--------|------|--------------------------------------|
/// | 0      | u8   | Trigger Action number                |
/// | 1      | u16  | Minimum activation time (in seconds) |
/// | 3      | u16  | Maximum activation time (in seconds) |
/// | 5      | u16  | Reaction time (in seconds)           |
/// | 7      | u8   | Param increase rate (slider val)     |
/// | 8      | u8   | Param decrease                       |
/// | 9      | u8   | Param decrease rate (slider val)     |
/// | 10     | u8   | Param increase                       |
/// | 11-13  | ...  | Padding (0x00)                       |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct RandomReactionTrigger {
        pub minimum_activation_time: u16,
        pub maximum_activation_time: u16,
        pub reaction_time: u16,
        pub param_increase_rate: u8,
        pub param_decrease: u8,
        pub param_decrease_rate: u8,
        #[deku(pad_bytes_after = "3")]
        pub param_increase: u8,
}

/// ### Trigger type 0x04 - Random Probability
/// | Offset | Type | Description                          |
/// |--------|------|--------------------------------------|
/// | 0      | u8   | Slot 1 Trigger Action                |
/// | 1      | u8   | Slot 1 Probability                   |
/// | 2      | u8   | Slot 2 Trigger Action                |
/// | 3      | u8   | Slot 2 Probability                   |
/// | 4      | u8   | Slot 3 Trigger Action                |
/// | 5      | u8   | Slot 3 Probability                   |
/// | 6      | u8   | Slot 4 Trigger Action                |
/// | 7      | u8   | Slot 4 Probability                   |
/// | 8      | u8   | Slot 5 Trigger Action                |
/// | 9      | u8   | Slot 5 Probability                   |
/// | 10     | u8   | Slot 6 Trigger Action                |
/// | 11     | u8   | Slot 6 Probability                   |
/// | 12     | u16  | Cooldown time (in seconds)           |
/// 
/// All probabilities are in increments of 0.5%. So for example, a value of 100 (0x64) corresponds to 50% probability. The maximum cooldown time the app can set is 18hr = 64800s = 0xfd20. It's not known if the device accepts higher values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct RandomProbabilityTrigger {
        pub slot1_trigger_action: u8,
        pub slot1_probability: u8,
        pub slot2_trigger_action: u8,
        pub slot2_probability: u8,
        pub slot3_trigger_action: u8,
        pub slot3_probability: u8,
        pub slot4_trigger_action: u8,
        pub slot4_probability: u8,
        pub slot5_trigger_action: u8,
        pub slot5_probability: u8,
        pub slot6_trigger_action: u8,
        pub slot6_probability: u8,
        pub cooldown_time: u16,
}

/// ### Trigger type 0x0F - External Voltage Input
/// | Offset | Type | Description                          |
/// |--------|------|--------------------------------------|
/// | 0      | u8   | Trigger Action number                |
/// | 1      | u8   | Active (0x01) or Passive (0x00)      |
/// | 2      | u8   | Minimum voltage                      |
/// | 3      | u8   | Maximum voltage                      |
/// | 4      | u8   | Param mapping max voltage            |
/// | 5-13   | ...  | Padding (0x00)                       |
/// 
/// The voltage values are in increments of 0.00875V. So for example, a value of 32 (0x20) corresponds to 0.28V, and a value of 64 (0x40) corresponds to 0.56V. The slider in the app goes up to 2.1v (0xF0), but it actually sends 0xFF for this value which is 2.23125V. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct ExternalVoltageInputTrigger {
    pub trigger_action_number: u8,
    pub is_active_voltage: bool,
    pub minimum_voltage_step: u8,
    pub maximum_voltage_step: u8,
    #[deku(pad_bytes_after = "9")]
    pub parameter_max_voltage_step: u8,
}

/// ## Command 0x40 - Update Accessory Settings
/// This command is sent when the user changes the accessory settings from the "Start/Stop Settings" dialog in the Accessory Settings screen. The device responds with a 0x41 notification.
///
/// ### Payload
/// | Offset | Type | Description                   |
/// |--------|------|-------------------------------|
/// | 0      | u16  | Delayed Start                 |
/// | 2      | u8   | Channel A Base Intensity      |
/// | 3      | u8   | Channel B Base Intensity      |
/// | 4      | u8   | Channel A Background Waveform |
/// | 5      | u8   | Channel B Background Waveform |
/// | 6      | u16  | Stop Timer Minimum Time       |
/// | 8      | u16  | Stop Timer Maximum Time       |
/// | 10     | u16  | Unknown                       |
/// | 12     | u8   | Dial Settings                 |
///
/// #### 0x00-0x01 - Delayed Start
/// The amount of time to delay before starting the play session. The maximum time the app can set is 18hr = 64800 seconds = 0xfd20. It's not known if the device accepts higher values.
/// - Offset: 0x00
/// - Size: 2 bytes
/// - Type: unsigned 16-bit integer
/// - Endianness: big endian
/// - Unit: seconds
/// - Range: 0 to 64800
///
/// #### 0x02 - Channel A Base Intensity
/// The base intensity for channel A. This will be the starting intensity when accessory gameplay starts. The maximum value the app can set is 100 (0x64). It's not known if the device accepts higher values. A value of 0xFF turns off this option.
/// - Offset: 0x02
/// - Size: 1 byte
/// - Type: unsigned 8-bit integer
/// - Unit: intensity level
/// - Range: 0 to 100, or 0xFF to disable
///
/// #### 0x03 - Channel B Base Intensity
/// The base intensity for channel B. This will be the starting intensity when accessory gameplay starts. The maximum value the app can set is 100 (0x64). It's not known if the device accepts higher values. A value of 0xFF turns off this option.
/// - Offset: 0x03
/// - Size: 1 byte
/// - Type: unsigned 8-bit integer
/// - Unit: intensity level
/// - Range: 0 to 100, or 0xFF to disable
///
/// #### 0x04 - Channel A Background Waveform
/// The background waveform for channel A. This will be the background waveform that plays during accessory gameplay. See [Waveform] for available waveforms.
/// - Offset: 0x04
/// - Size: 1 byte
/// - Type: unsigned 8-bit integer
/// - Unit: Waveform index
/// - Range: see [Waveform]
///
/// #### 0x05 - Channel B Background Waveform
/// The background waveform for channel B. This will be the background waveform that plays during accessory gameplay. See [Waveform] for available waveforms.
/// - Offset: 0x05
/// - Size: 1 byte
/// - Type: unsigned 8-bit integer
/// - Unit: Waveform index
/// - Range: see [Waveform]
///
/// #### 0x06 - Play Session Stop Timer - Minimum Time
/// The minimum time for the stop timer. For Fixed mode, set the minimum time and maximum time to the same value. For Random mode, the stop time is randomly chosen between the minimum and maximum time. The maximum value the app can set is 13h30m = 48600s = 0xBDB8. It's not known if the device accepts higher values.
/// - Offset: 0x06
/// - Size: 2 bytes
/// - Type: unsigned 16-bit integer
/// - Endianness: big endian
/// - Unit: seconds
/// - Range: 0 to 48600
///
/// #### 0x08 - Play Session Stop Timer - Maximum Time
/// The maximum time for the stop timer. For Fixed mode, set the minimum time and maximum time to the same value. For Random mode, the stop time is randomly chosen between the minimum and maximum time. The maximum value the app can set is 13h30m = 48600s = 0xBDB8. It's not known if the device accepts higher values.
/// - Offset: 0x08
/// - Size: 2 bytes
/// - Type: unsigned 16-bit integer
/// - Endianness: big endian
/// - Unit: seconds
/// - Range: 0 to 48600
///
/// #### 0x0A - Unknown
/// Unknown 2-byte value. Always 0x0000 in my testing.
///
/// #### 0x0C - Dial Settings
/// A set of bits determining the dial behavior.
///
/// Fields, from left to right (MSB first):
/// - bit 0: Unused
/// - bit 1: Unused
/// - bit 2: Unused
/// - bit 3: Unused
/// - bit 4: Unused
/// - bit 5: Unused
/// - bit 6: Disable Intensity Adjustment
/// - bit 7: Disable Shutdown
///
/// ##### Disable Intensity Adjustment via Knobs
/// Prevents using the shoulder dials to changing the output intensity during an active play session.
///
/// ##### Disable Shutdown via Knobs
/// Prevents using the shoulder dials to shut down the device during an active play session. Normally, holding both shoulder dials will shut down the device.
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command40UpdateAccessorySettings {
    #[deku(endian = "big")]
    pub delayed_start: u16,
    pub channel_a_base_intensity: BaseIntensity,
    pub channel_b_base_intensity: BaseIntensity,
    pub channel_a_background_waveform: Waveform,
    pub channel_b_background_waveform: Waveform,
    #[deku(endian = "big")]
    pub stop_timer_minimum_time: u16,
    #[deku(endian = "big")]
    pub stop_timer_maximum_time: u16,
    #[deku(endian = "big")]
    pub unknown_1: u16,
    #[deku(pad_bits_before = "6", bits = 1)]
    pub disable_intensity_adjustment: bool,
    #[deku(bits = 1)]
    pub disable_shutdown: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum BaseIntensity {
    #[deku(id = "0xFF")]
    Off,
    #[deku(id_pat = "_")]
    Level(u8),
}

/// ## Command 0x50 - Unknown
/// Sent after connecting to the device. Also sent when changing the slot color in the 4.0 app.
///
/// ### Payload
/// | Offset | Type | Description           |
/// |--------|------|-----------------------|
/// | 0      | u8   | Indicator Color       |
/// | 1      | ...  | Unknown               |
///
/// First byte is the indicator color - see [Color]. The rest of the payload is unknown.
/// 
/// The 3.0 app doen't have a color option and always sends `5007000000000000000000000000000000`.
/// 
/// ### Example
/// ```
/// 0x50 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
/// ```
///
/// ### Response
/// The device responds with a [0x51 - Unknown] notification.
/// 
/// [0x51 - Unknown]: crate::v3::notifications::Notification51Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command50Unknown {
    pub indicator_color: Color,
    #[deku(read_all)]
    pub unknown_data: Vec<u8>,
}

/// ## Command 0x60 - Accessory Command
/// This command is sent when the user changes a pawprint's shoulder lights or clears the trigger parameters.
///
/// ### Payload
/// The payload is the button ID (1 byte) followed by a subcommand (1 byte) and subcommand data.
/// 
/// | Offset | Type | Description           |
/// |--------|------|-----------------------|
/// | 0      | u8   | Button ID             |
/// | 1      | u8   | Subcommand            |
/// | ...    | ...  | Subcommand Data       |
/// 
/// ### Subcommands
/// 
/// #### Subcommand 0x70 - Set Light Color
/// Changes the shoulder light color of a pawprint.
///
/// | Offset | Type | Description          |
/// |--------|------|----------------------|
/// | 0      | u8   | Button ID            |
/// | 1      | u8   | Set Light Color      |
/// | 2      | u8   | Color                |
///
/// For color values, see [Color]
/// 
/// ##### Example
/// `60 01 70 07` - Set button 1's shoulder light color to cyan.
///
/// #### Subcommand 0x5F - Clear Trigger Parameters
/// Clears the trigger parameters for a button
///
/// | Offset | Type | Description               |
/// |--------|------|---------------------------|
/// | 0      | u8   | Button ID                 |
/// | 1      | u8   | Clear Triggers SubCommand |
/// 
/// ##### Example
/// `60 01 5F` - Clear button 1's trigger parameters.
/// 
/// ##### Subcommand 0x60 - Detect Angle Thresholds
/// Triggers the automatic angle threshold detection for a button. The device will respond with a [0x70 - Angle Threshold Detected] notification.
/// 
///
/// ```
/// 0x60 01 60
/// ```
/// ### Response
/// The device responds with a [0x61 - Unknown] notification.
/// 
/// [0x61 - Unknown]: crate::v3::notifications::Notification61Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command60AccessoryCommand {}

/// ## Command 0xA0 - Unknown
/// Sent when the user saves a waveform to device memory. It is followed by one or more `0xA2` commands containing the waveform data.
///
/// ### Payload
/// The payload is 6 bytes long, with unknown content.
///
/// - 0: Seems to match the number of 0xA2 commands that follow containing the waveform data
/// - 1: The slot to save the waveform to, from 1 to 8
/// - 2,3: Unknown, seems to stay constant/stable across different waveforms and slots
/// - 4,5: Unknown, changes with each waveform but not slot - might be a checksum of some kind?
///
/// | Offset | Type | Example | Description |
/// |--------|------|---------|-------------|
/// | 0      | u8   | 0x02    | Unknown     |
/// | 1      | u8   | 0x06    | Slot #      |
/// | 2      | u8   | 0x00    | Unknown     |
/// | 3      | u8   | 0x14    | Unknown     |
/// | 4      | u8   | 0xC4    | Unknown     |
/// | 5      | u8   | 0x1E    | Unknown     |
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandA0Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Command 0xA2 - Unknown
/// Sent one or more times after a `0xA0` command, containing waveform data to save to device memory.
///
/// ### Payload
/// Unknown, variable size.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandA2Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Command 0xAF - Restore Waveforms
/// Factory restores the on-board waveform modes in the app.
///
/// ### Payload
/// This command has no payload.
///
/// ### Response
/// The device responds with a [0xAE - Waveforms Restored] notification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandAFRestoreWaveforms {}

/// ## Command 0xB0 - Set Intensity and Waveform Data
/// > The B0 instruction writes the channel intensity change and channel waveform data. The instruction data length is 20 bytes and is written every 100ms. The data of both channels are in the same instruction.
///
/// ### Payload
/// The payload is 19 bytes long.
/// - (4 bits) serial number
/// - (4 bits) intensity value interpretation method
/// - (1 byte) A channel intensity setting value
/// - (1 byte) B channel intensity setting value
/// - (4 bytes) A channel waveform frequency 4 lines
/// - (4 bytes) A channel waveform intensity 4 lines
/// - (4 bytes) B channel waveform frequency 4 lines
/// - (4 bytes) B channel waveform intensity 4 lines
///
/// ### Response
/// - If the intensity changed, the device will respond with a [0xB1 - Intensity Changed] notification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandB0SetIntensity {
    /// 4 bits, 0b0000 ~ 0b1111
    ///
    /// > The serial number range is (0b0000 ~ 0b1111). If the channel strength of the pulse host is modified in the input data, set the serial number>0, and the pulse host will return the modified channel strength from the characteristic 0x150B with the same serial number through the B1 response message. If the pulse host does not need to feedback the channel strength, set the serial number to 0b0000. In addition, to avoid problems, when the channel strength is modified through the B0 instruction and the serial number is not 0, it is recommended to wait for 150B to return B1 with the same serial number information before modifying the channel strength.
    #[deku(bits = 4)]
    pub serial_number: u8,
    /// 2 bits, 0b00 ~ 0b11
    ///
    /// > The 4 bits of intensity value interpretation are divided into two parts. The upper two bits represent the interpretation of channel A, and the lower two bits represent the interpretation of channel B.
    pub intensity_interpretation_a: IntensityInterpretation,
    /// 2 bits, 0b00 ~ 0b11
    ///
    /// > The 4 bits of intensity value interpretation are divided into two parts. The upper two bits represent the interpretation of channel A, and the lower two bits represent the interpretation of channel B.
    pub intensity_interpretation_b: IntensityInterpretation,
    /// 1 byte, 0 ~ 200, values over 200 are treated as 0
    ///
    /// > The channel strength setting value is 1 byte long, with a valid range of (0~200). Values ​​outside the input range are treated as 0. The absolute range of the strength of each channel of the Coyote host is also (0~200).
    pub channel_a_intensity: u8,
    /// 1 byte, 0 ~ 200, values over 200 are treated as 0
    ///
    /// > The channel strength setting value is 1 byte long, with a valid range of (0~200). Values ​​outside the input range are treated as 0. The absolute range of the strength of each channel of the Coyote host is also (0~200).
    pub channel_b_intensity: u8,
    /// 4 intervals, 1 byte each, 10 ~ 240
    pub waveform_a_freq: [u8; 4],
    /// 4 intervals, 1 byte each, 0 ~ 100
    pub waveform_a_intensity: [u8; 4],
    /// 4 intervals, 1 byte each, 10 ~ 240
    pub waveform_b_freq: [u8; 4],
    /// 4 intervals, 1 byte each, 0 ~ 100
    pub waveform_b_intensity: [u8; 4],
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8", bits = 2)]
pub enum IntensityInterpretation {
    /// > Represents that the intensity of the corresponding channel does not change. The intensity setting value of the corresponding channel is invalid no matter what it is.
    Ignore = 0x00,
    /// > Represents the relative increase of the intensity of the corresponding channel. If the intensity setting value of channel A is 15 (decimal), then the intensity of channel A increases by 15.
    Increase = 0x01,
    /// > Represents the relative decrease of the intensity of the corresponding channel. If the intensity setting value of channel A is 17 (decimal), then the intensity of channel A decreases by 17.
    Decrease = 0x10,
    /// > Represents the absolute change of the intensity of the corresponding channel. If the intensity setting value of channel A is 32, then the intensity of channel A is set to 32.
    Absolute = 0x11,
}

/// ## Command 0xBC - Unknown
/// Sent when adjusting frequency balance parameters. Device responds with a 0xBD notification.
///
/// ### Payload
/// Unknown. The second and third bytes change when switching between soft and standard mode.
///
/// ### Example
/// Value: 0xbc 04 32 32 32 1e 50 96 32 - standard mode
/// Value: 0xbc 04 28 3c 32 1e 50 96 32 - soft mode
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandBCUnknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Command 0xBF - Set Balance and Limits
/// > The BF command writes the channel strength soft upper limit + waveform frequency balance parameter + waveform strength balance parameter of the pulse host. The command data length is 7 bytes.
///
/// ### Payload
/// The payload is 6 bytes long.
/// - (1 byte) A channel strength soft upper limit
/// - (1 byte) B channel strength soft upper limit
/// - (1 byte) A channel waveform frequency balance parameter
/// - (1 byte) B channel waveform frequency balance parameter
/// - (1 byte) A channel waveform strength balance parameter
/// - (1 byte) B channel waveform strength balance parameter
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandBFSetLimits {
    /// 1 byte, 0 ~ 200, invalid values are ignored
    ///
    /// > Limits the maximum value that the pulse host channel strength can reach, and the setting is saved when the power is off. The value range is (0~200). The value outside the input range will not modify the soft upper limit.
    pub intensity_limit_a: u8,
    /// 1 byte, 0 ~ 200, invalid values are ignored
    ///
    /// > Limits the maximum value that the pulse host channel strength can reach, and the setting is saved when the power is off. The value range is (0~200). The value outside the input range will not modify the soft upper limit.
    pub intensity_limit_b: u8,
    /// 1 byte, 0 ~ 255
    ///
    /// > Adjusts the feeling of the high and low frequencies of the waveform, and the setting is saved when the power is turned off. The value range is (0 ~ 255).
    pub frequency_balance_a: u8,
    /// 1 byte, 0 ~ 255
    ///
    /// > Adjusts the feeling of the high and low frequencies of the waveform, and the setting is saved when the power is turned off. The value range is (0 ~ 255).
    pub frequency_balance_b: u8,
    /// 1 byte, 0 ~ 255
    ///
    /// > Adjusts the waveform pulse width, and the setting is saved when the power is turned off. The value range is (0 ~ 255).
    pub intensity_balance_a: u8,
    /// 1 byte, 0 ~ 255
    ///
    /// > Adjusts the waveform pulse width, and the setting is saved when the power is turned off. The value range is (0 ~ 255).
    pub intensity_balance_b: u8,
}

/// ## Command 0xFE - Clear Accessory Data
///
/// Sent when clearing accessory settings. Clears all trigger actions and conditions, and unbinds all accessories.
///
/// The device then responds with a series of notifications (0xF1, 0xF2, 0xF3, 0xF4) containing the current settings.
///
/// ### Payload
/// This command has no payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandFEClearAccessoryData {}

/// ## Command 0xFF - Get All Accessory Data
///
/// Sent when refreshing accessory settings in the 4.0 app.
///
/// The device then responds with a series of notifications:
/// - 0xF1
/// - 0xF3 - once for each configured trigger condition
/// - 0xF2 - once for each configured trigger action
/// - 0xF4
///
/// ### Payload
/// This command has no payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandFFGetAllAccessoryData {}

/// Represents an unknown command.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct UnknownCommand {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Command {
    #[deku(id = "0x01")]
    PairPawprint(Command01PairPawprint),
    #[deku(id = "0x08")]
    UnpairPawprint(Command08UnpairPawprint),
    #[deku(id = "0x0e")]
    ManualBroadcastingMode(Command0EManualBroadcastingMode),
    #[deku(id = "0x0c")]
    Unknown0C(Command0CUnknown),
    #[deku(id = "0x0D")]
    Unknown0D(Command0DUnknown),
    #[deku(id = "0x11")]
    StartAccessoryGameplay(Command11StartAccessoryGameplay),
    #[deku(id = "0x12")]
    StopAccessoryGameplay(Command12StopAccessoryGameplay),
    #[deku(id = "0x20")]
    SetTriggerAction(Command20SetTriggerAction),
    #[deku(id = "0x28")]
    DeleteTriggerAction(Command28DeleteTriggerAction),
    #[deku(id = "0x30")]
    SetTriggerCondition(Command30SetTriggerCondition),
    #[deku(id = "0x40")]
    UpdateAccessorySettings(Command40UpdateAccessorySettings),
    #[deku(id = "0x50")]
    Unknown50(Command50Unknown),
    #[deku(id = "0x60")]
    Unknown60(Command60AccessoryCommand),
    #[deku(id = "0xA0")]
    UnknownA0(CommandA0Unknown),
    #[deku(id = "0xA2")]
    UnknownA2(CommandA2Unknown),
    #[deku(id = "0xAF")]
    RestoreWaveforms(CommandAFRestoreWaveforms),
    #[deku(id = "0xB0")]
    SetIntensity(CommandB0SetIntensity),
    #[deku(id = "0xBC")]
    UnknownBC(CommandBCUnknown),
    #[deku(id = "0xBF")]
    SetLimits(CommandBFSetLimits),
    #[deku(id = "0xFE")]
    ClearAccessoryData(CommandFEClearAccessoryData),
    #[deku(id = "0xFF")]
    GetAllAccessoryData(CommandFFGetAllAccessoryData),
    #[deku(id_pat = "_")]
    Unknown(u8, UnknownCommand),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_enum_01() {
        let data = vec![0x01];
        let ((rest, _), command) = Command::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(rest, &b""[..]);
        assert_eq!(command, Command::PairPawprint(Command01PairPawprint {}));
    }

    #[test]
    fn test_command_enum_20() {
        let data = vec![
            0x20, 0x02, 0x03, 0x07, 0x0d, 0x28, 0xe3, 0x13, 0x02, 0xf0, 0xff, 0xf9, 0x03, 0xfa,
            0x05, 0xf7, 0xcc, 0xf7, 0xcc, 0x07,
        ];
        let ((rest, _), command) = Command::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(rest, &b""[..]);
        assert_eq!(
            command,
            Command::SetTriggerAction(Command20SetTriggerAction {
                trigger_id: 0x02,
                allow_repeated_triggering: true,
                stop_play_session_immediately: true,
                skip_delay_and_start_immediately: true,
                temporary_output_waveform_channel_a: Waveform::BuiltInMode7,
                temporary_output_waveform_channel_b: Waveform::PresetPunishmentWaveform6,
                temporary_intensity_change_channel_a_mode: TemporaryIntensityChangeMode::Gradient(
                    37
                ),
                temporary_intensity_change_channel_a_lower_bound: -29,
                temporary_intensity_change_channel_a_upper_bound: 19,
                temporary_intensity_change_channel_b_mode: TemporaryIntensityChangeMode::Random,
                temporary_intensity_change_channel_b_lower_bound: -16,
                temporary_intensity_change_channel_b_upper_bound: -1,
                permanent_intensity_change_channel_a_lower_bound: -7,
                permanent_intensity_change_channel_a_upper_bound: 3,
                permanent_intensity_change_channel_b_lower_bound: -6,
                permanent_intensity_change_channel_b_upper_bound: 5,
                adjust_remaining_session_time_lower_bound: -35 * 60,
                adjust_remaining_session_time_upper_bound: -35 * 60,
                temporary_trigger_mode: TemporaryTriggerMode::Delay(7),
            })
        );
    }

    #[test]
    fn test_command_enum_unknown() {
        let data = vec![0xAA, 0x21, 0x22, 0x23];
        let ((rest, _), command) = Command::from_bytes((data.as_ref(), 0)).unwrap();
        assert_eq!(rest, &b""[..]);
        assert_eq!(
            command,
            Command::Unknown(
                0xAA,
                UnknownCommand {
                    data: vec![0x21, 0x22, 0x23]
                }
            )
        );
    }
}
