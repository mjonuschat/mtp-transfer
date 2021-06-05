use clap::{AppSettings, Clap, ValueHint};
use std::path::PathBuf;
use thiserror::Error;

static ACTIVITY_DIR: &str = "GARMIN/Activity";

#[derive(Clap, Debug)]
pub struct Sync {
    /// Select device by name - if multiple MTP devices are attached
    #[clap(short, long)]
    pub device: Option<String>,
    /// Path to the activity files on the MTP device
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath, default_value = ACTIVITY_DIR)]
    pub input: PathBuf,
    /// Path to where the downloaded activities are being written
    #[clap(short, long, parse(from_os_str), value_hint = ValueHint::DirPath, default_value = ".")]
    pub output: PathBuf,
    /// Force export and overwrite all existing files
    #[clap(short, long)]
    pub force: bool,
}

impl Sync {
    pub fn activity_dir(&self) -> PathBuf {
        self.input.to_owned()
    }
}

#[derive(Clap, Debug)]
pub struct MtpFileTree {
    /// Select device by model name
    #[clap(short, long, conflicts_with_all=&["manufacturer", "serial"])]
    pub model: Option<String>,
    /// Select device by manufacturer name
    #[clap(short='a', long, conflicts_with_all=&["model", "serial"])]
    pub manufacturer: Option<String>,
    /// Select device by serial number
    #[clap(short, long, conflicts_with_all=&["model", "manufacturer"])]
    pub serial: Option<String>,
    /// Show all files and folders
    #[clap(long)]
    pub all: bool,
}

#[derive(Clap, Debug)]
pub enum MtpCommand {
    #[clap(about = "Detect MTP devices")]
    Detect,
    #[clap(about = "Show tree of folders and activity files", name = "filetree")]
    FileTree(MtpFileTree),
}

#[derive(Clap, Debug)]
#[clap(author, about, version, setting = AppSettings::ColorAuto, setting = AppSettings::ColoredHelp)]
pub struct Mtp {
    #[clap(subcommand)]
    pub command: MtpCommand,
}

#[derive(Clap, Debug)]
pub enum Command {
    #[clap(about = "Download activity files")]
    Sync(Sync),
    #[clap(about = "MTP device tools and diagnostics")]
    Mtp(Mtp),
}

#[derive(Clap, Debug)]
#[clap(author, about, version, name = "Garmin Activity Downloader", setting=AppSettings::ColorAuto, setting=AppSettings::ColoredHelp)]
pub struct ApplicationArguments {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Path `{0}` could not be resolved")]
    Canonicalize(#[from] std::io::Error),
    #[error("File or directory `{0}` is not accessible")]
    Inaccessible(String),
}
