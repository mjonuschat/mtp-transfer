use crate::mtp;
use crate::types::DeviceSelector;
use anyhow::Result;

pub fn detect(verbose: u8) -> Result<()> {
    mtp::detect(verbose)
}

pub fn filetree(selector: DeviceSelector, verbose: bool) -> Result<()> {
    mtp::filetree(selector, verbose)
}
