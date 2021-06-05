use crate::mtp;
use anyhow::Result;

pub fn detect(verbose: u8) -> Result<()> {
    mtp::detect(verbose)
}
