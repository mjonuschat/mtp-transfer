use clap::{AppSettings, Clap, ValueHint};
use std::path::PathBuf;
use thiserror::Error;

static ACTIVITY_DIR: &str = "GARMIN/Activity";

#[derive(Clap, Debug)]
#[clap(author, about, version, name = "Garmin Activity Downloader", setting=AppSettings::ColorAuto, setting=AppSettings::ColoredHelp)]
pub struct Options {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,
    /// Select device by name - if multiple MTP devices are attached
    #[clap(short, long)]
    pub device: Option<String>,
    /// Path to the activity files on the MTP device
    #[clap(short, long, parse(from_os_str), value_hint=ValueHint::DirPath, default_value=ACTIVITY_DIR)]
    pub input: PathBuf,
    /// Path to where the downloaded activities are being written
    #[clap(short, long, parse(from_os_str), value_hint=ValueHint::DirPath, default_value=".")]
    pub output: PathBuf,
    /// Force export and overwrite all existing files
    #[clap(short, long)]
    pub force: bool,
    /// Download activity files with this extension
    #[clap(short, long, default_value = ".fit")]
    pub extension: String,
}

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Path `{0}` could not be resolved")]
    Canonicalize(#[from] std::io::Error),
    #[error("File or directory `{0}` is not accessible")]
    Inaccessible(String),
}

impl Options {
    pub fn activity_dir(&self) -> PathBuf {
        self.input.to_owned()
    }
}
