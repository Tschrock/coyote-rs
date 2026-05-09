//! Pawprint Host Commands
//!
//! Commands are formatted as a single byte identifier followed by a variable length payload.
//!
//! ```
//! +-----+------+------+------+------>
//! | ID  | DATA                  ... |
//! +-----+------+------+------+------>
//! ```
//!
//! ## Device Commands
//!
//! | Command                                    | Description                            |
//! |--------------------------------------------|----------------------------------------|
//! | [0x50](Command50Unknown)                   | Unknown                                |
//! | [0x5F](Command5FUnknown)                   | Unknown                                |
//! | [0x60](Command60DetectAngles)              | Detect Angles                          |
//! | [0x70](Command70SetColor)                  | Set Color                              |
//!
//! To test:
//! - 0x57
//! - 0xFF
//!

use deku::prelude::*;
use serde::{Deserialize, Serialize};

use crate::v3::common::Color;

/// ## Command 0x50 - Unknown
/// Sent when a trigger condition is set.
/// 
/// ## Examples
/// 
/// - 50 01 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 - Cleared all conditions
/// - 50 01 01 01 00 00 00 00 00 00 00 00 00 00 00 00 00 - Press/Release, Press
/// - 50 01 01 01 00 15 3c 2c 12 00 00 00 00 00 00 00 00 - Press/Release, Press, Param Inc 18, Param Inc Rate 21s, Param Dec 60, Param Dec Rate 44s
/// - 50 01 01 01 01 00 00 00 00 00 00 00 00 00 00 00 00 - Press/Release, Release
/// - 50 01 02 00 00 00 01 00 00 00 07 00 00 00 00 00 00 - Acceleration/Angle, Mode 1, <7
/// - 50 01 02 00 00 00 01 00 01 01 04 00 00 00 00 00 00 - Acceleration/Angle, Mode 1, >260
/// - 50 01 02 00 00 00 01 02 da 1c ee 35 bb 04 00 00 00 - Acceleration/Angle, Mode 2, X Min -76, X Max 56, Y Min -36, Y Max 106, Z Min -138, Z Max 8
/// - 
/// 
/// ### Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command50Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Command 0x60 - Detect Angles
/// Starts the angle detection process, which automatically detects the angle thresholds for the Acceleration/Angle trigger condition.
/// 
/// ### Payload
/// This command has no payload.
/// 
/// ### Responses
/// The device immediately responds with a [0x51 - Unknown]. After 10 seconds, the device responds with an [0xF1 - Detect Angles Result].
/// 
/// [0x51 - Unknown]: crate::pawprint::notifications::Notification51Unknown
/// [0xF1 - Detect Angles Result]: crate::pawprint::notifications::NotificationF1DetectAnglesResult
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command60DetectAngles {}

/// ## Command 0x70 - Set Color
/// Sets the color of the shoulder lights.
///
/// ### Payload
/// | Offset | Type | Description           |
/// |--------|------|-----------------------|
/// | 0      | u8   | Color                 |
///
/// ### Responses
/// - [0x51 - Unknown]
///
/// [0x51 - Unknown]: crate::pawprint::notifications::Notification51Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Command70SetColor {
    pub color: Color,
}
