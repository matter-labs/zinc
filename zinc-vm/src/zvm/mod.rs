//!
//! The Zinc virtual machine binary.
//!

mod arguments;
mod error;

use std::process;

use structopt::StructOpt;

use crate::arguments::Arguments;
use crate::arguments::Command;

static BINARY_NAME: &str = "zvm";

fn main() {
    let args = Arguments::from_args();

    zinc_utils::logger::init_logger(BINARY_NAME, args.verbosity);

    let result = match args.command {
        Command::Run(command) => command.execute(),
        Command::Debug(command) => command.execute(),
        Command::Test(command) => match command.execute() {
            Ok(status) => {
                process::exit(status as i32);
            }
            Err(error) => {
                eprintln!("{}", error);
                process::exit(zinc_const::exit_code::FAILURE);
            }
        },
        Command::Setup(command) => command.execute(),
        Command::Prove(command) => command.execute(),
        Command::Verify(command) => command.execute(),
    };

    match result {
        Ok(()) => process::exit(zinc_const::exit_code::SUCCESS),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(zinc_const::exit_code::FAILURE);
        }
    }
}
