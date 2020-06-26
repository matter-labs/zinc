//!
//! The Zargo project manager binary.
//!

mod command;
mod directory;
mod executable;
mod manifest;

use std::process;

use structopt::StructOpt;

use self::command::error::Error as CommandError;
use self::command::Command;

#[derive(Debug, StructOpt)]
#[structopt(name = "zargo", about = "Zinc's project manager")]
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
        Ok(()) => zinc_const::exit_code::SUCCESS,
        Err(error) => {
            log::error!("{}", error);
            zinc_const::exit_code::FAILURE
        }
    })
}

fn main_inner(args: Arguments) -> Result<(), CommandError> {
    zinc_utils::logger::init_logger("zargo", args.verbosity);

    args.command.execute()
}
