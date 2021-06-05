use crate::arguments;
use crate::arguments::MtpFileTree;

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

impl From<&arguments::Sync> for DeviceSelector {
    fn from(params: &arguments::Sync) -> Self {
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
