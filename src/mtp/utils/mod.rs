use crate::mtp::{DeviceSelector, MtpError};

use libmtp_rs::device::{
    raw::{detect_raw_devices, RawDevice},
    MtpDevice,
};
use libmtp_rs::error::{Error as FfiMtpError, MtpErrorKind};

mod detect;
mod filetree;

pub use detect::detect;
pub use filetree::filetree;

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

pub fn get_device(selector: &DeviceSelector) -> Result<MtpDevice, MtpError> {
    let raw_devices = get_raw_devices()?;

    if raw_devices.len() > 1 && matches!(selector, DeviceSelector::First) {
        println!(
            "Found {} MTP devices, defaulting to first one found.",
            raw_devices.len()
        );
        println!("Please select a specific device using manufacturer/model/serial number");
    }

    for raw_device in raw_devices {
        if let Some(device) = raw_device.open_uncached() {
            match selector {
                DeviceSelector::First => return Ok(device),
                DeviceSelector::ManufacturerName(ref pattern) => {
                    if let Ok(name) = device.manufacturer_name() {
                        if name.contains(pattern) {
                            return Ok(device);
                        }
                    }
                }
                DeviceSelector::ModelName(ref pattern) => {
                    if let Ok(name) = device.model_name() {
                        if name.contains(pattern) {
                            return Ok(device);
                        }
                    }
                }
                DeviceSelector::SerialNumber(ref pattern) => {
                    if let Ok(serial) = device.serial_number() {
                        if serial == *pattern {
                            return Ok(device);
                        }
                    }
                }
            }
        } else {
            let device = raw_device.device_entry();
            println!(
                "Could not open device (Vendor {:04x}, Product {:04x}), skipping...",
                device.vendor_id, device.product_id
            )
        }
    }

    Err(MtpError::DeviceNotFound)
}
