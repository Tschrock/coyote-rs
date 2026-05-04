//! BLE interface for the Coyote Pulse Host V2.
//!
//! <https://github.com/DG-LAB-OPENSOURCE/DG-LAB-OPENSOURCE/blob/main/coyote/v2/README_V2.md>
//!
//! ## Service/Characteristics Table
//!
//! | Service UUID | Characteristic UUID | Attributes        | Name          | Size (BYTE) |
//! |--------------|---------------------|-------------------|---------------|-------------|
//! |       0x180A |              0x1500 | Read/Notify       | Battery_Level | 1 Byte      |
//! |       0x180B |              0x1504 | Read/Write/Notify | PWM_AB2       | 3 bytes     |
//! |       0x180B |              0x1505 | Read/Write        | PWM_A34       | 3 bytes     |
//! |       0x180B |              0x1506 | Read/Write        | PWM_B34       | 3 bytes     |
//!
//!

pub const PULSE_HOST_NAME: &str = "D-LAB ESTIM01";
