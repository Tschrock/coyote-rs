//! BLE interface for the Coyote PawPrint.
//!
//! <https://github.com/DG-LAB-OPENSOURCE/DG-LAB-OPENSOURCE/blob/main/coyote/v3/README_V3.md>
//!
//! ## Service/Characteristics Table
//!
//! | Service                     | Characteristic                      | Properties   | Value Size      | Description                                               |      Example Value |
//! |-----------------------------|-------------------------------------|--------------|-----------------|-----------------------------------------------------------|--------------------|
//! | 0x1800 - Generic Access     | 0x2A00 - Device Name                | Read, Write  | Up to 248 bytes | Device name                                               |        "47L120100" |
//! | 0x1801 - Generic Attribute  | 0x2A05 - Service Changed            | Read, Indicate | 4 bytes       | Service Changed                                           |         0x0100FFFF |
//! | 0x180C - PawPrint           | 0x150A - Device Command             | Write        | Up to 20 bytes  | All commands are input in this characteristic             |                    |
//! | 0x180C - PawPrint           | 0x150B - Device Notification        | Read, Notify | Up to 20 bytes  | All response messages are returned in this characteristic |                    |
//! | F000FFC0-0451-4000-B000-000000000000 - TI SensorTag OvertheAir Download | F000FFC1-0451-4000-B000-000000000000 - Image Identify | Write No Response, Write, Notify | Unknown | TODO: read TI docs | |
//! | F000FFC0-0451-4000-B000-000000000000 - TI SensorTag OvertheAir Download | F000FFC2-0451-4000-B000-000000000000 - Img Block | Write No Response, Write, Notify | Unknown | TODO: read TI docs | |
//! 
//! TODO: figure out how to pair and maintain connection. Most communication happens between the Coyote and the PawPrint so we can't just snoop the android app.
//! 

pub const PAWPRINT_DEFAULT_NAME: &str = "47L120100";
