//! BLE interface for the Coyote PawPrint.
//!
//! <https://github.com/DG-LAB-OPENSOURCE/DG-LAB-OPENSOURCE/blob/main/coyote/v3/README_V3.md>
//!

pub mod attributes;
pub mod commands;
pub mod common;
pub mod device;
pub mod notifications;

/// The default name for the PawPrint.
pub const PAWPRINT_DEFAULT_NAME: &str = "47L120100";
