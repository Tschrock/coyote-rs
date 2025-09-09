pub struct CoyoteV3Device {
    pub name: Option<String>,
    pub mac: Option<String>,
    pub battery: Option<u8>,
    pub label: Option<u8>,
    pub version: Option<u8>,
}

impl CoyoteV3Device {
    pub async fn get_label(&self) -> u8 {
        match self.label {
            Some(label) => label,
            None => {
                let (label, _) = self.read_label_and_version().await;
                label
            }
        }
    }
    pub async fn get_version(&self) -> u8 {
        match self.version {
            Some(version) => version,
            None => {
                let (_, version) = self.read_label_and_version().await;
                version
            }
        }
    }
    pub async fn read_label_and_version(&self) -> (u8, u8) {
        todo!("read label and version from ble characteristic and cache it")
    }
    pub async fn get_battery(&self) -> u8 {
        match self.battery {
            Some(battery) => battery,
            None => {
                let battery = self.read_battery().await;
                battery
            }
        }
    }
    pub async fn read_battery(&self) -> u8 {
        todo!("read battery from ble characteristic")
    }
    pub async fn subscribe_battery(&self) {
        todo!("subscribe to battery characteristic")
    }
    pub async fn read_name(&self) -> String {
        todo!("read name from ble characteristic")
    }
    pub async fn send_command(&self) -> String {
        todo!("send command")
    }
}
