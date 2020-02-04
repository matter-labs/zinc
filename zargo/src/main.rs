//!
//! The Zargo circuit manager binary.
//!

mod command;
mod constants;
mod manifest;
mod templates;

pub use self::command::Command;
pub use self::command::Error as CommandError;

use std::process;

use log::LevelFilter;
use structopt::StructOpt;

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
    verbose: usize,
    #[structopt(subcommand)]
    command: Command,
}

fn main() {
    let args: Arguments = Arguments::from_args();
    init_logger(args.verbose);

    process::exit(match main_inner(args) {
        Ok(()) => EXIT_CODE_SUCCESS,
        Err(error) => {
            log::error!("{}", error);
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner(args: Arguments) -> Result<(), CommandError> {
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
