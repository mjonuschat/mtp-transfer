use crate::mtp::MtpError;

use libmtp_rs::device::raw::{detect_raw_devices, RawDevice};
use libmtp_rs::error::{Error as FfiMtpError, MtpErrorKind};

mod detect;

pub use detect::detect;

pub(super) fn get_raw_devices() -> Result<Vec<RawDevice>, MtpError> {
    detect_raw_devices().map_err(|e| match e {
        FfiMtpError::Unknown => MtpError::FfiError(e),
        FfiMtpError::Utf8Error { .. } => MtpError::FfiError(e),
        FfiMtpError::MtpError { kind, .. } => match kind {
            MtpErrorKind::NoDeviceAttached => MtpError::NoDeviceAttached,
            _ => MtpError::FfiError(e),
        },
    })
}
