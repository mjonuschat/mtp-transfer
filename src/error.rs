use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Folder {0} could not be found")]
    FolderNotFound(String),
}

#[derive(Debug, Error)]
pub enum PathError {
    #[error("Path `{0}` could not be resolved")]
    Canonicalize(#[from] std::io::Error),
    #[error("File or directory `{0}` is not accessible")]
    Inaccessible(String),
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("No MTP device found on USB bus")]
    NoDeviceAttached,
    #[error("No device matching selection criteria found")]
    DeviceNotFound,
    #[error("FFI error: {0}")]
    LibMtpError(#[from] libmtp_rs::error::Error),
}
