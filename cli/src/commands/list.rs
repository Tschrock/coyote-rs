use btleplug::{
    api::{Central, Manager as _},
    platform::Manager,
};
use thiserror::Error;

use crate::commands::util::Plural;

#[derive(Error, Debug)]
pub enum ListAdaptersError {
    #[error("Error initializing bluetooth manager: {0}")]
    ManagerError(btleplug::Error),
    #[error("Error getting adapters: {0}")]
    AdapterError(btleplug::Error),
    #[error("Error getting info for adapter {1}: {0}")]
    AdapterInfoError(btleplug::Error, usize),
}

pub async fn list_adapters() -> Result<(), ListAdaptersError> {
    // Get the bluetooth manager
    let manager = match Manager::new().await {
        Ok(manager) => manager,
        Err(e) => return Err(ListAdaptersError::ManagerError(e)),
    };

    // Get the list of adapters
    let adapters = match manager.adapters().await {
        Ok(adapters) => adapters,
        Err(e) => return Err(ListAdaptersError::AdapterError(e)),
    };

    // Print out info for each adapter
    let count = adapters.len();
    println!("Found {} adapter{}:", count, count.plural("", "s"));

    for (index, adapter) in adapters.iter().enumerate() {
        let info = adapter
            .adapter_info()
            .await
            .map_err(|e| ListAdaptersError::AdapterInfoError(e, index))?;
        println!("{}: {:?}", index, info);
    }
    println!("\nUse \"--adapter <index>\" to select one. For example:");
    println!("$ coyote-connect scan --adapter 0");
    Ok(())
}
