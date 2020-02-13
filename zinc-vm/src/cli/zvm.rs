mod commands;

use crate::commands::{Arguments, Command};
use std::process::exit;
use structopt::StructOpt;

mod errors;
pub use errors::*;

fn main() {
    let args = Arguments::from_args();

    zinc_bytecode::logger::init_logger(args.verbosity);

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
