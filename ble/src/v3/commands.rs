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
//! | Command | Length   | Description                                               | Data Example                                                  |
//! |---------|----------|-----------------------------------------------------------|---------------------------------------------------------------|
//! | 0x01    | 1 byte   | Triggers pawprint pairing                                 | 0x01                                                          |
//! | 0x08    | 2 byte   | Unknown                                                   | 0x08 01                                                       |
//! | 0x20    | 19 bytes | Set trigger action                                        | 0x20 ...                                                      |
//! | 0x28    | 2 bytes  | Delete trigger action                                     | 0x28 02                                                       |
//! | 0x30    | 19 bytes | Set trigger condition                                     | 0x30 ...                                                      |
//! | 0x50    | 17 bytes | Unknown                                                   | 0x50 07 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00          |
//! | 0x60    | 4 bytes  | Set color or clear trigger parameters.                    | 0x60 01 70 07                                                 |
//! | 0xAF    | 1 byte   | Restore factory waveforms                                 | 0xAF                                                          |
//! | 0xB0    | 20 bytes | Sets the intensity and waveform data.                     | 0xB0 00 00 00 00 00 00 00 00 00 00 FF 00 00 00 00 00 00 00 FF |
//! | 0xBF    | 7 bytes  | Sets intensity limits and waveform balance parameters.    | 0xBF 64 64 A0 A0 00 00                                        |
//! | 0xFE    | 1 byte   | Clears accessory data                                     | 0xFE                                                          |
//! | 0xFF    | 1 byte   | Unknown - get all pawprint settings?                      | 0xFF                                                          |

use std::io::{Read, Seek, Write};
use deku::{bitvec::BitVec, ctx::Endian, prelude::*};
use serde::{Deserialize, Serialize};

/// ## Command 0x01 - Pair pawprint
/// This command is sent to the Coyote when the user taps the button to add a pawprint in the app.
///
/// Once the coyote finds and adds the pawprint, it will respond with an `0x01` notification.
/// If a pawprint is not found within 10 seconds, the coyote will stop searching and respond with an `0x02` notification.
///
/// ### Payload
/// This command has no payload.
///
/// ### Example
/// ```
/// 0x01
/// ```
/// 
/// ### Responses
/// - `0x01` Notification: Pawprint successfully paired.
/// - `0x02` Notification: Pawprint pairing timed out.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command01PairPawprint {}

/// ## Command 0x08 - Unknown
/// TODO: Document this command.
/// 
/// ### Payload
/// The payload is 1 byte long. Unknown content.
/// 
/// ### Example
/// ```
/// 0x08 0x01
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command08Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Command 0x20 - Set trigger action
///
/// This command is sent when the user configures a trigger action in the app.
///
///
/// ### Payload
/// 
/// The payload is 19 bytes long.
///
/// | Offset | Type | Default | Description                                             |
/// |--------|------|---------|---------------------------------------------------------|
/// | 0x00   | u8   | 0x20    | Command ID, always 0x00.                                |
/// | 0x01   | u8   | 0x01    | Trigger ID.                                             |
/// | 0x02   | u8   | 0x10    | Bit map containing some of the boolean trigger options. |
/// | 0x03   | u8   | 0x00    | Temporary output waveform for channel A.                |
/// | 0x04   | u8   | 0x00    | Temporary output waveform for channel B.                |
/// | 0x05   | u8   | 0x00    | Temporary intensity change mode for channel A.          |
/// | 0x06   | i8   | 0x00    | Temporary intensity change lower bound for channel A.   |
/// | 0x07   | i8   | 0x00    | Temporary intensity change upper bound for channel A.   |
/// | 0x08   | u8   | 0x00    | Temporary intensity change mode for channel B.          |
/// | 0x09   | i8   | 0x00    | Temporary intensity change lower bound for channel B.   |
/// | 0x0A   | i8   | 0x00    | Temporary intensity change upper bound for channel B.   |
/// | 0x0B   | i8   | 0x00    | Permanent intensity change lower bound for channel A.   |
/// | 0x0C   | i8   | 0x00    | Permanent intensity change upper bound for channel A.   |
/// | 0x0D   | i8   | 0x00    | Permanent intensity change lower bound for channel B.   |
/// | 0x0E   | i8   | 0x00    | Permanent intensity change upper bound for channel B.   |
/// | 0x0F   | i16  | 0x00    | Adjust remaining session time lower bound.              |
/// | 0x11   | i16  | 0x00    | Adjust remaining session time upper bound.              |
/// | 0x13   | u8   | 0x00    | Temporary trigger delay mode.                           |
///
/// ### Options Bit Map (offset 0x02)
/// Each bit corresponds to a boolean option. From left to right (MSB first), they are:
/// - bit 0: Unknown
/// - bit 1: Unknown
/// - bit 2: Unknown
/// - bit 3: Allow repeated triggering (inverted)
/// - bit 4: Unknown
/// - bit 5: Unknown
/// - bit 6: Stop play session immediately
/// - bit 7: Skip delay and start immediately
///
/// ### Temporary Output Waveform (offsets 0x03 and 0x04)
/// The value is an unsigned 8-bit integer that corresponds to a waveform saved in device memory.
///
/// The available values are:
/// - 0x00: No waveform
/// - 0x01: Built-in mode1
/// - 0x02: Built-in mode2
/// - 0x03: Built-in mode3
/// - 0x04: Built-in mode4
/// - 0x05: Built-in mode5
/// - 0x06: Built-in mode6
/// - 0x07: Built-in mode7
/// - 0x08: Built-in mode8
/// - 0x09: Preset Background Waveform1
/// - 0x0A: Preset Background Waveform2
/// - 0x0B: Preset Punishment Waveform1
/// - 0x0C: Preset Punishment Waveform2
/// - 0x0D: Preset Punishment Waveform3
/// - 0x0E: Preset Punishment Waveform4
/// - 0x0F: Preset Punishment Waveform5
/// - 0x10: Preset Punishment Waveform6
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
/// ### Temporary Intensity Change Mode (offsets 0x05 and 0x08)
/// The intensity change mode is a single byte. It packs together both the mode and (in Gradient mode) the gradient value.
/// - 0x01: Fixed
/// - 0x02: Random
/// - 0x03: Parameter Mapping
/// - 0x04-0xFF: Gradient, with the gradient value being (x - 3)
///
/// ### Temporary Intensity Change Lower/Upper Bound (offsets 0x06, 0x07, 0x09, and 0x0A)
/// These are signed 8-bit integers. The lower bound must be less than or equal to the upper bound.
///
/// - In `Fixed` mode, both the lower and upper bounds are the same value.
/// - In `Random` mode, the value uses is randomized between the lower and upper bounds.
/// - In `Parameter Mapping` mode, the value is mapped between the lower and upper bounds based on the trigger parameter.
/// - In `Gradient` mode, the value gradually changes from the lower to the upper bound, changing by 1 every (gradient value) seconds.
///
/// TODO: What happens if the mode is `Fixed` but the upper and lower bounds are different?
///
/// ### Permanent Intensity Change Lower/Upper Bounds (offsets 0x0B-0x0E)
/// These are signed 8-bit integers. The lower bound must be less than or equal to the upper bound.
///
/// When the values are the same, the intensity change operates in `Fixed` mode. When different, the intensity change operates in `Random` mode.
/// 
/// ### Example
/// ```hex
/// 0x20 02 03 07 0d 28 e3 13 02 f0 ff f9 03 fa 05 f7 cc f7 cc 07
/// ```
/// 
/// | Offset | Description                   | Raw Value | Value                       |
/// |--------|-------------------------------|-----------|-----------------------------|
/// | 0      | Command ID                    | 0x20      | 0x20                        |
/// | 1      | Trigger ID                    | 0x02      | 2                           |
/// | 2      | Options                       | 0x03      | 0b00000011                  |
/// | 3      | Temp waveform ch A            | 0x07      | Built-in mode7              |
/// | 4      | Temp waveform ch B            | 0x0D      | Preset Punishment Waveform3 |
/// | 5      | Temp intensity change A mode  | 0x28      | Gradient(37)                |
/// | 6      | Temp intensity change A lower | 0xE3      | -29                         |
/// | 7      | Temp intensity change A upper | 0x13      | 19                          |
/// | 8      | Temp intensity change B mode  | 0x02      | Random                      |
/// | 9      | Temp intensity change B lower | 0xF0      | -16                         |
/// | 10     | Temp intensity change B upper | 0xFF      | -1                          |
/// | 11     | Perm intensity change A lower | 0xF9      | -7                          |
/// | 12     | Perm intensity change A upper | 0x03      | 3                           |
/// | 13     | Perm intensity change B lower | 0xFA      | -6                          |
/// | 14     | Perm intensity change B upper | 0x05      | 5                           |
/// | 15     | Adjust time lower bound       | 0xF7CC    | -35 minutes                 |
/// | 17     | Adjust time upper bound       | 0xF7CC    | -35 minutes                 |
/// | 19     | Temporary trigger mode        | 0x07      | Delay(7)                    |
/// 
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command20SetTriggerAction {
    pub trigger_id: u8,
    #[deku(bits = 1)]
    pub unknown_option_1: bool,
    #[deku(bits = 1)]
    pub unknown_option_2: bool,
    #[deku(bits = 1)]
    pub unknown_option_3: bool,
    #[deku(
        reader = "read_bit_inverted(deku::reader)",
        writer = "write_bit_inverted(deku::writer, &self.allow_repeated_triggering)"
    )]
    pub allow_repeated_triggering: bool,
    #[deku(bits = 1)]
    pub unknown_option_5: bool,
    #[deku(bits = 1)]
    pub unknown_option_6: bool,
    #[deku(bits = 1)]
    pub stop_play_session_immediately: bool,
    #[deku(bits = 1)]
    pub skip_delay_and_start_immediately: bool,
    pub temporary_output_waveform_channel_a: u8,
    pub temporary_output_waveform_channel_b: u8,
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
/// ```
/// 0x28 0x02
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command28DeleteTriggerAction {
    pub trigger_id: u8,
}

/// ## Command 0x30 - Set trigger condition
/// This command is sent when the user configures a trigger condition in the app.
/// 
/// ### Payload
/// The payload is 19 bytes long.
/// 
/// TODO: Document the payload.
/// 
/// ### Example
/// ```
/// 0x30 01 0f 03 01 10 30 20 00 00 00 00 00 00 00 00 00
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command30SetTriggerCondition {}

/// ## Command 0x60 - Unknown
/// This command is sent when the user changes a pawprint's shoulder lights or clears the trigger parameters.
/// 
/// ### Payload
/// 
/// For changing the shoulder lights, the payload is 3 bytes long.
/// 
/// ```
/// 0x60 01 70 07
/// ```
/// 
/// | Offset | Type | Example | Description |
/// |--------|------|---------|-------------|
/// | 0      | u8   | 0x60    | Command ID  |
/// | 1      | u8   | 0x01    | Button ID   |
/// | 2      | u8   | 0x70    | Unknown     |
/// | 3      | u8   | 0x07    | Color       |
/// 
/// For color values, see [Color]
/// 
/// For clearing the trigger parameters, the payload is 2 bytes long.
/// 
/// ```
/// 0x60 01 5F
/// ```
/// 
/// | Offset | Type | Example | Description |
/// |--------|------|---------|-------------|
/// | 0      | u8   | 0x60    | Command ID  |
/// | 1      | u8   | 0x01    | Button ID   |
/// | 2      | u8   | 0x5F    | Unknown     |
/// 
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command60Unknown {}

/// ## Command 0xAF - Restore waveforms
///
/// This command is sent when the user factory restores the on-board waveform modes in the app.
///
/// This command has no payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct CommandAFRestoreWaveforms {}


/// ## Command 0xB0 - Set intensity
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


/// ## Command 0xBF - Set limits
/// 
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    #[deku(id = "0x20")]
    SetTriggerAction(Command20SetTriggerAction),
    #[deku(id = "0x28")]
    DeleteTriggerAction(Command28DeleteTriggerAction),
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
                unknown_option_1: false,
                unknown_option_2: false,
                unknown_option_3: false,
                allow_repeated_triggering: true,
                unknown_option_5: false,
                unknown_option_6: false,
                stop_play_session_immediately: true,
                skip_delay_and_start_immediately: true,
                temporary_output_waveform_channel_a: 7,
                temporary_output_waveform_channel_b: 13,
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
            Command::Unknown(0xAA, UnknownCommand {
                data: vec![0x21, 0x22, 0x23]
            })
        );
    }
}
