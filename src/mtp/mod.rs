// Add types here

mod devices;
mod files;
mod storage;
mod utils;

pub use devices::select_device;
pub use files::get_files;
pub use storage::select_storage;
pub use utils::{detect, filetree, get_device};

use crate::arguments::MtpFileTree;
use libmtp_rs::{device::MtpDevice, storage::Parent};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MtpError {
    #[error("No MTP device found on USB bus")]
    NoDeviceAttached,
    #[error("Multiple MTP device found")]
    MultipleDevicesFound,
    #[error("No device matching selection criteria found")]
    DeviceNotFound,
    #[error("Could not open device: {0}")]
    IoError(String),
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
