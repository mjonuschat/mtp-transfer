use crate::mtp::{Device, MtpError};
use libmtp_rs::device::raw::detect_raw_devices;
use libmtp_rs::error::{Error as FfiMtpError, MtpErrorKind};
use libmtp_rs::storage::Parent;

pub fn select_device(_pattern: &str) -> Result<Device, MtpError> {
    let raw_devices = match detect_raw_devices() {
        Ok(devices) => devices,
        Err(e) => {
            return match e {
                FfiMtpError::Unknown => Err(MtpError::FfiError(e)),
                FfiMtpError::Utf8Error { .. } => Err(MtpError::FfiError(e)),
                FfiMtpError::MtpError { kind, .. } => match kind {
                    MtpErrorKind::NoDeviceAttached => Err(MtpError::NoDeviceAttached),
                    _ => Err(MtpError::FfiError(e)),
                },
            }
        }
    };

    if raw_devices.len() > 1 {
        return Err(MtpError::MultipleDevicesFound);
    }

    if let Some(raw_device) = raw_devices.into_iter().next() {
        return if let Some(mtp_device) = raw_device.open_uncached() {
            let name = match mtp_device.get_friendly_name() {
                Ok(fname) => fname,
                Err(_) => format!(
                    "{} {}",
                    mtp_device.manufacturer_name()?,
                    mtp_device.model_name()?
                ),
            };

            println!("Found MTP device: {}", &name);
            Ok(Device {
                name,
                handle: mtp_device,
                storage: 0,
                activity_folder: Parent::Root,
            })
        } else {
            let device = raw_device.device_entry();

            Err(MtpError::IoError(format!(
                "Vendor {:04x} {:04x}",
                device.vendor_id, device.product_id
            )))
        };
    }

    Err(MtpError::NoDeviceAttached)
}
