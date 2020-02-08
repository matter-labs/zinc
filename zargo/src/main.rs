//!
//! The Zargo circuit manager binary.
//!

mod command;
mod directory;
mod executable;
mod manifest;

use std::process;

use log::LevelFilter;
use structopt::StructOpt;

use self::command::error::Error as CommandError;
use self::command::Command;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

#[derive(Debug, StructOpt)]
#[structopt(name = "zargo", about = "Zinc's circuit manager")]
struct Arguments {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,
    #[structopt(subcommand)]
    command: Command,
}

fn main() {
    let args: Arguments = Arguments::from_args();

    process::exit(match main_inner(args) {
        Ok(()) => EXIT_CODE_SUCCESS,
        Err(error) => {
            log::error!("{}", error);
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner(args: Arguments) -> Result<(), CommandError> {
    init_logger(args.verbosity);

    args.command.execute()
}

fn init_logger(verbosity: usize) {
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(match verbosity {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .init();
}
