//! Pawprint Host notifications
//!
//! Notifications are formatted as a single byte identifier followed by a variable length payload.
//!
//! 
//! ```
//! +-----+------+------+------+------>
//! | ID  | DATA                  ... |
//! +-----+------+------+------+------>
//! ```
//!
//! | Notification                  | Description        |
//! |-------------------------------|--------------------|
//! | [0x51](Notification51Unknown) | Unknown            |
//! | [0x53](Notification53Unknown) | Unknown            |
//! | [0xF1](NotificationF1DetectAnglesResult) | Detect Angles Result |

use deku::{DekuRead, DekuWrite};
use serde::{Deserialize, Serialize};

/// ## Notification 0x51 - Unknown
/// Unknown
/// 
/// ### Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification51Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0x53 - Unknown
/// Unknown
/// 
/// ### Payload
/// Unknown
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Notification53Unknown {
    #[deku(read_all)]
    pub data: Vec<u8>,
}

/// ## Notification 0xF1 - Detect Angles Result
/// Result of the angle detection process started by the [0x60 - Detect Angles] command. Might be used for more responses, but 0x61 is the only known one so far.
/// 
/// ### Payload
/// | Offset | Type | Description          |
/// |--------|------|----------------------|
/// | 0      | u8   | Response Type (0x61) |
/// | 1      | i16  | X Min                |
/// | 3      | i16  | X Max                |
/// | 5      | i16  | Y Min                |
/// | 7      | i16  | Y Max                |
/// | 9      | i16  | Z Min                |
/// | 11     | i16  | Z Max                |
/// 
/// ### Examples
/// - f1 61 ff d9 00 29 ff c6 ff ff 00 5e 00 88 - X Min -39, X Max 41, Y Min -58, Y Max 0, Z Min 94, Z Max 136
/// 
/// [0x60 - Detect Angles]: crate::pawprint::commands::Command60DetectAngles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct NotificationF1DetectAnglesResult {
    pub response_type: u8,
    pub x_min: i16,
    pub x_max: i16,
    pub y_min: i16,
    pub y_max: i16,
    pub z_min: i16,
    pub z_max: i16,
}
