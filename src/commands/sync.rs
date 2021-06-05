use crate::arguments::Sync;
use crate::{helpers, mtp};

use std::borrow::Borrow;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use libmtp_rs::device::MtpDevice;

fn friendly_name(device: &MtpDevice) -> String {
    match device.get_friendly_name() {
        Ok(fname) => fname,
        Err(_) => format!(
            "{} {}",
            device
                .manufacturer_name()
                .unwrap_or_else(|_| "Unknown".to_string()),
            device
                .model_name()
                .unwrap_or_else(|_| "Unknown".to_string())
        ),
    }
}

fn serial_number(device: &MtpDevice) -> String {
    device
        .serial_number()
        .unwrap_or_else(|_| "Unknown".to_string())
}

pub fn run(options: &Sync) -> Result<()> {
    let device: MtpDevice = mtp::get_device(&options.into())?;
    let storage_pool = device.storage_pool();
    let dst_folder = options.output.join(format!(
        "{} - {}",
        friendly_name(&device),
        serial_number(&device)
    ));

    let activity_folder =
        mtp::find_activity_folder(storage_pool.borrow(), &options.activity_dir())?;

    let files = mtp::get_files(activity_folder.storage, activity_folder.folder);

    let total_progress = ProgressBar::new(files.len() as u64);
    total_progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40} {pos:>7}/{len:7} {msg}"),
    );

    helpers::create_output_dir(&dst_folder)?;
    let existing = helpers::read_existing_activities(&dst_folder);

    for file in files {
        total_progress.set_message(&&file.name().to_string());

        if !existing.contains(&file.name().to_string()) {
            let dst = dst_folder.join(&file.name());

            activity_folder.storage.get_file_to_path(file, dst)?;
        }

        total_progress.inc(1);
    }
    total_progress.finish();

    Ok(())
}
