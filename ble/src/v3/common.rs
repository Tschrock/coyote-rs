use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};


/// ## Colors
/// The colors used by the pawprint LEDs.
/// 
/// | Value | Color  |
/// |-------|--------|
/// | 0x00  | Off    |
/// | 0x01  | Yellow |
/// | 0x02  | Red    |
/// | 0x03  | Purple |
/// | 0x04  | Blue   |
/// | 0x05  | Cyan   |
/// | 0x06  | Green  |
/// | 0x07  | White  |
/// 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Color {
    #[deku(id = "0x00")]
    Off,
    #[deku(id = "0x01")]
    Yellow,
    #[deku(id = "0x02")]
    Red,
    #[deku(id = "0x03")]
    Purple,
    #[deku(id = "0x04")]
    Blue,
    #[deku(id = "0x05")]
    Cyan,
    #[deku(id = "0x06")]
    Green,
    #[deku(id = "0x07")]
    White,
}

/// ## Waveforms
/// The waveforms used for accessory configuration and trigger actions.
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
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum Waveform {
    NoWaveform = 0x00,
    BuiltInMode1 = 0x01,
    BuiltInMode2 = 0x02,
    BuiltInMode3 = 0x03,
    BuiltInMode4 = 0x04,
    BuiltInMode5 = 0x05,
    BuiltInMode6 = 0x06,
    BuiltInMode7 = 0x07,
    BuiltInMode8 = 0x08,
    PresetBackgroundWaveform1 = 0x09,
    PresetBackgroundWaveform2 = 0x0A,
    PresetPunishmentWaveform1 = 0x0B,
    PresetPunishmentWaveform2 = 0x0C,
    PresetPunishmentWaveform3 = 0x0D,
    PresetPunishmentWaveform4 = 0x0E,
    PresetPunishmentWaveform5 = 0x0F,
    PresetPunishmentWaveform6 = 0x10,
}
