// Add types here

mod files;
mod storage;
mod utils;

pub use files::get_activity_files;
pub use storage::find_activity_folder;
pub use utils::{detect, filetree, get_device};

use crate::arguments::{MtpFileTree, Sync};
use libmtp_rs::{device::MtpDevice, storage::Parent};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MtpError {
    #[error("No MTP device found on USB bus")]
    NoDeviceAttached,
    #[error("No device matching selection criteria found")]
    DeviceNotFound,
    #[error("FFI error: {0}")]
    FfiError(#[from] libmtp_rs::error::Error),
}

#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub handle: MtpDevice,
    pub storage: u32,
    pub activity_folder: Parent,
}

#[derive(Debug)]
pub enum DeviceSelector {
    First,
    ManufacturerName(String),
    ModelName(String),
    SerialNumber(String),
}

impl From<MtpFileTree> for DeviceSelector {
    fn from(params: MtpFileTree) -> Self {
        if let Some(serial) = params.serial {
            DeviceSelector::SerialNumber(serial)
        } else if let Some(model) = params.model {
            DeviceSelector::ModelName(model)
        } else if let Some(manufacturer) = params.manufacturer {
            DeviceSelector::ManufacturerName(manufacturer)
        } else {
            DeviceSelector::First
        }
    }
}

impl From<&Sync> for DeviceSelector {
    fn from(params: &Sync) -> Self {
        if let Some(serial) = &params.serial {
            DeviceSelector::SerialNumber(serial.to_owned())
        } else if let Some(model) = &params.model {
            DeviceSelector::ModelName(model.to_owned())
        } else if let Some(manufacturer) = &params.manufacturer {
            DeviceSelector::ManufacturerName(manufacturer.to_owned())
        } else {
            DeviceSelector::First
        }
    }
}
