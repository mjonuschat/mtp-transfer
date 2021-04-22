use crate::arguments::PathError;
use anyhow::{Context, Result};
use clap::Clap;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

mod arguments;
mod mtp;

fn create_output_dir(path: &Path) -> Result<(), PathError> {
    match std::fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(PathError::Inaccessible(path.to_string_lossy().to_string()))
            }
        }
        Err(_e) => Ok(std::fs::create_dir_all(&path)?),
    }
}

fn main() -> Result<()> {
    let options = arguments::Options::parse();
    let pattern = options
        .device
        .as_ref()
        .map_or_else(|| "".to_string(), |v| v.to_owned());

    let device = crate::mtp::select_storage(
        crate::mtp::select_device(&pattern)?,
        &options.activity_dir(),
    )?;
    let storage_pool = device.handle.storage_pool();
    let storage = storage_pool
        .by_id(device.storage)
        .context("Couldn't open storage")?;

    let files = crate::mtp::get_files(storage, device.activity_folder, &options.extension);

    let total_progress = ProgressBar::new(files.len() as u64);
    total_progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40} {pos:>7}/{len:7} {msg}"),
    );

    let dst_folder = options.output.join(format!(
        "{} - {}",
        &device.name,
        &device
            .handle
            .serial_number()
            .unwrap_or_else(|_| "Unknown".to_string())
    ));
    create_output_dir(&dst_folder)?;

    for file in files {
        total_progress.set_message(&&file.name().to_string());
        let dst = dst_folder.join(&file.name());

        storage.get_file_to_path(file, dst)?;

        total_progress.inc(1);
    }
    total_progress.finish();

    Ok(())
}
