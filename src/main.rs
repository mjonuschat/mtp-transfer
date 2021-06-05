use anyhow::Result;
use clap::Clap;

mod arguments;
mod commands;
mod mtp;
mod output;

fn main() -> Result<()> {
    let options = arguments::ApplicationArguments::parse();

    match options.command {
        arguments::Command::Mtp(params) => match params.command {
            arguments::MtpCommand::Detect => commands::mtp::detect(options.verbose),
        },
        arguments::Command::Sync(params) => commands::sync::run(&params),
    }
}
