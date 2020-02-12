mod commands;

use crate::commands::{Arguments, Command};
use log::LevelFilter;
use std::process::exit;
use structopt::StructOpt;

mod errors;
pub use errors::*;

fn main() {
    let args = Arguments::from_args();

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(match args.verbosity {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .init();

    let result = match args.command {
        Command::Run(command) => command.execute(),
        Command::Setup(command) => command.execute(),
        Command::Prove(command) => command.execute(),
        Command::Verify(command) => command.execute(),
    };

    if let Err(error) = result {
        log::error!("{}", error);
        exit(1);
    }
}
