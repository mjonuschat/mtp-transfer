// Add types here

mod devices;
mod files;
mod storage;
mod utils;

pub use devices::select_device;
pub use files::get_files;
pub use storage::select_storage;
pub use utils::detect;

use libmtp_rs::{device::MtpDevice, storage::Parent};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MtpError {
    #[error("No MTP device found")]
    NoDeviceAttached,
    #[error("Multiple MTP device found")]
    MultipleDevicesFound,
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
