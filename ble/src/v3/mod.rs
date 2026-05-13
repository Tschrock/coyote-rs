//! BLE interface for the Coyote Pulse Host V3.

pub mod attributes;
pub mod commands;
pub mod common;
pub mod device;
pub mod notifications;

/// The default name for the Pulse Host.
pub const PULSE_HOST_DEFAULT_NAME: &str = "47L121000";

/// The default name for the Pulse Host when in recovery/dfu mode (hold both knob buttons when powered off - eyes should light up solid blue). In this mode, only the GAP, GATT, and DFU services are available.
pub const PULSE_HOST_DFU_DEFAULT_NAME: &str = "47L121000_O3";
