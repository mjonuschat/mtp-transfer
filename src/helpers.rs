use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;

const ACTIVITY_FILE_TYPES: [&str; 3] = ["fit", "gpx", "tcx"];

pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ])
            .template("{msg} {spinner:.blue}"),
    );
    pb.set_message(msg);
    pb
}

pub fn is_activity_file(file: &str) -> bool {
    let extension = Path::new(&file.to_lowercase())
        .extension()
        .and_then(|v| v.to_str().map(|v| v.to_owned()));

    match extension {
        Some(ext) => ACTIVITY_FILE_TYPES.contains(&ext.as_str()),
        None => false,
    }
}
