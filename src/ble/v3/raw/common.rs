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
