use crate::v3::{attributes::*, commands::Command};
use btleplug::api::{BDAddr, Characteristic, Peripheral, WriteType};
use deku::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub enum DeviceEvent {
    BatteryLevel(u8),
    CommandResponse(),
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error(transparent)]
    BtError(#[from] btleplug::Error),
    #[error(transparent)]
    DekuError(#[from] deku::DekuError),
    #[error("{0}: {1:?}")]
    DecodeError(String, Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct CoyoteV3Device<P: Peripheral> {
    peripheral: P,
}

impl<P: Peripheral> CoyoteV3Device<P> {
    fn find(
        &self,
        service_uuid: uuid::Uuid,
        characteristic_uuid: uuid::Uuid,
    ) -> Result<Characteristic, btleplug::Error> {
        self.peripheral
            .services()
            .iter()
            .find(|s| s.uuid == service_uuid)
            .ok_or(btleplug::Error::NoSuchCharacteristic)?
            .characteristics
            .iter()
            .find(|c| c.service_uuid == service_uuid && c.uuid == characteristic_uuid)
            .cloned()
            .ok_or(btleplug::Error::NoSuchCharacteristic)
    }
    async fn read(
        &self,
        service_uuid: uuid::Uuid,
        characteristic_uuid: uuid::Uuid,
    ) -> Result<Vec<u8>, btleplug::Error> {
        let characteristic = self.find(service_uuid, characteristic_uuid)?;
        self.peripheral.read(&characteristic).await
    }
    async fn read_deku<T: for<'a> deku::DekuContainerRead<'a>>(
        &self,
        service_uuid: uuid::Uuid,
        characteristic_uuid: uuid::Uuid,
    ) -> Result<T, DeviceError> {
        let data = self.read(service_uuid, characteristic_uuid).await?;
        let (remainder, value) = T::from_bytes((data.as_slice(), data.len()))?;
        if remainder.1 != 0 {
            return Err(DeviceError::DecodeError(
                format!(
                    "Unexpected data from characteristic {} - Expected {} bytes, got {}",
                    characteristic_uuid,
                    data.len() - remainder.1,
                    data.len()
                ),
                data,
            ));
        }
        Ok(value)
    }
    async fn write_deku<T: deku::DekuContainerWrite>(
        &self,
        service_uuid: uuid::Uuid,
        characteristic_uuid: uuid::Uuid,
        value: &T,
        write_type: WriteType,
    ) -> Result<(), DeviceError> {
        let bytes = value.to_bytes()?;
        let characteristic = self.find(service_uuid, characteristic_uuid)?;
        self.peripheral
            .write(&characteristic, &bytes, write_type)
            .await?;
        Ok(())
    }
}

impl<P: Peripheral> CoyoteV3Device<P> {
    pub fn new(peripheral: P) -> Self {
        Self { peripheral }
    }

    /// Reads the device name from the device
    ///
    /// #### Service
    ///
    /// [Generic Access Service (0x1800)][crate::v3::attributes#0x1800---gap-service]
    ///
    /// #### Characteristic
    ///
    /// [Device Name (0x2A00)][crate::v3::attributes#0x2a00---device-name-characteristic]
    ///
    /// #### Data
    ///
    /// UTF-8 formated string, up to 248 bytes long
    ///
    pub async fn read_device_name(&self) -> Result<String, DeviceError> {
        let characteristic =
            self.find(SERVICE_GENERIC_ACCESS_PROFILE, CHARACTERISTIC_DEVICE_NAME)?;
        let data = self.peripheral.read(&characteristic).await?;
        String::from_utf8(data).map_err(|e| {
            DeviceError::DecodeError(format!("Failed to decode device name: {}", e), Vec::new())
        })
    }

    /// Reads the battery level from the device
    ///
    /// #### Service
    ///
    /// [Device Information Service (0x180A)][crate::v3::attributes#0x180a---device-information-service]
    ///
    /// #### Characteristic
    ///
    /// [Battery Level (0x1500)][crate::v3::attributes#0x1500---battery-level-characteristic]
    ///
    /// #### Data
    ///
    /// One byte representing the battery level as a percentage (0-100)
    ///
    pub async fn read_battery(&self) -> Result<u8, DeviceError> {
        let characteristic = self.find(SERVICE_DEVICE_INFORMATION, CHARACTERISTIC_BATTERY)?;
        let data = self.peripheral.read(&characteristic).await?;
        if data.len() != 1 {
            return Err(DeviceError::DecodeError(
                format!(
                    "Unexpected data from Battery Characteristic - Expected 1 byte, got {}",
                    data.len()
                ),
                data,
            ));
        }
        if data[0] > 100 {
            return Err(DeviceError::DecodeError(
                "Battery level out of range (0-100)".into(),
                data,
            ));
        }
        Ok(data[0])
    }

    /// Subscribes to battery level notifications
    ///
    /// #### Service
    ///
    /// [Device Information Service (0x180A)][crate::v3::attributes#0x180a---device-information-service]
    ///
    /// #### Characteristic
    ///
    /// [Battery Level (0x1500)][crate::v3::attributes#0x1500---battery-level-characteristic]
    ///
    pub async fn subscribe_battery(&self) -> Result<(), DeviceError> {
        let characteristic = self.find(SERVICE_DEVICE_INFORMATION, CHARACTERISTIC_BATTERY)?;
        self.peripheral.subscribe(&characteristic).await?;
        Ok(())
    }

    /// Reads the label and version from the device
    ///
    /// #### Service
    ///
    /// [Device Information Service (0x180A)][crate::v3::attributes#0x180a---device-information-service]
    ///
    /// #### Characteristic
    ///
    /// [Version (0x1501)][crate::v3::attributes#0x1501---version-characteristic]
    ///
    /// #### Data
    ///
    /// Two bytes: label and version
    ///
    pub async fn read_version(&self) -> Result<DeviceLabelVersion, DeviceError> {
        self.read_deku(SERVICE_DEVICE_INFORMATION, CHARACTERISTIC_VERSION)
            .await
    }

    /// Reads the mac address of the device
    ///
    /// #### Service
    ///
    /// [Device Information Service (0x180A)][crate::v3::attributes#0x180a---device-information-service]
    ///
    /// #### Characteristic
    ///
    /// [MAC Address (0x1502)][crate::v3::attributes#0x1502---mac-address-characteristic]
    ///
    /// #### Data
    ///
    /// Six bytes representing the Bluetooth MAC address of the device in big-endian format (e.g. `00:11:22:33:44:55` would be represented as `[0x00, 0x11, 0x22, 0x33, 0x44, 0x55]`)
    ///
    pub async fn read_address(&self) -> Result<BDAddr, DeviceError> {
        let characteristic = self.find(SERVICE_DEVICE_INFORMATION, CHARACTERISTIC_MAC)?;
        let data = self.peripheral.read(&characteristic).await?;
        data.as_slice().try_into().map_err(|_| {
            DeviceError::DecodeError(
                format!(
                    "Unexpected data from Address characteristic - Expected 6 bytes, got {}",
                    data.len()
                ),
                data,
            )
        })
    }

    /// Reads the unknown device information characteristic 0x2A59 from the device
    ///
    /// #### Service
    ///
    /// [Device Information Service (0x180A)][crate::v3::attributes#0x180a---device-information-service]
    ///
    /// #### Characteristic
    ///
    /// [Unknown Characteristic (0x2A59)][crate::v3::attributes#0x2a59---unknown-characteristic]
    ///
    /// #### Data
    ///
    /// Unknown - See characteristic documentation for details
    ///
    pub async fn read_2a59(&self) -> Result<DeviceInformation2A59, DeviceError> {
        self.read_deku(SERVICE_DEVICE_INFORMATION, CHARACTERISTIC_UNKNOWN_2A59)
            .await
    }

    /// Sends a command to the device
    ///
    /// ### Service
    ///
    /// [Pulse Host Service (0x180C)][crate::v3::attributes#0x180c---pulse-host-service]
    ///
    /// ### Characteristic
    ///
    /// [Device Command (0x2A60)][crate::v3::attributes#0x2a60---device-command-characteristic]
    ///
    pub async fn send_command(&self, command: Command) -> Result<(), DeviceError> {
        self.write_deku(
            SERVICE_PULSE_HOST,
            CHARACTERISTIC_DEVICE_COMMAND,
            &command,
            WriteType::WithResponse,
        )
        .await
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum LoadDetection {
    NoOutput = 0x01,
    Connected = 0x02,
    Disconnected = 0x03,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct DeviceInformation2A59 {
    load_detection_a: LoadDetection,
    load_detection_b: LoadDetection,
    unknown_1_a: u8,
    unknown_1_b: u8,
    unknown_2_a: u8,
    unknown_2_b: u8,
    unknown_3: u8,
    unknown_4: u8,
    unknown_5: u8,
    unknown_6: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct DeviceLabelVersion {
    label: u8,
    version: u8,
}
