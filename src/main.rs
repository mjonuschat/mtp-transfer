use anyhow::Result;
use clap::Clap;

mod arguments;
mod commands;
mod error;
mod helpers;
mod mtp;
mod types;

fn main() -> Result<()> {
    let options = arguments::ApplicationArguments::parse();

    match options.command {
        arguments::Command::Mtp(params) => match params.command {
            arguments::MtpCommand::Detect => commands::mtp::detect(options.verbose),
            arguments::MtpCommand::FileTree(params) => {
                let verbose = params.all;
                commands::mtp::filetree(params.into(), verbose)
            }
        },
        arguments::Command::Sync(params) => commands::sync::run(&params),
    }
}
