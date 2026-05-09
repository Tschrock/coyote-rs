//! BLE interface for the Coyote PawPrint.
//!
//! <https://github.com/DG-LAB-OPENSOURCE/DG-LAB-OPENSOURCE/blob/main/coyote/v3/README_V3.md>
//!

pub mod device;
pub mod attributes;
pub mod commands;
pub mod notifications;
pub mod common;

/// The default name for the PawPrint.
pub const PAWPRINT_DEFAULT_NAME: &str = "47L120100";
