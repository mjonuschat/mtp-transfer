use libmtp_rs::{device::MtpDevice, storage::Parent};
use thiserror::Error;

pub use files::get_activity_files;
pub use storage::find_activity_folder;
pub use utils::{detect, filetree, get_device};

mod files;
mod storage;
mod utils;

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
