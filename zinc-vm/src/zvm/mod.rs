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

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

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
                process::exit(EXIT_CODE_FAILURE);
            }
        },
        Command::Setup(command) => command.execute(),
        Command::Prove(command) => command.execute(),
        Command::Verify(command) => command.execute(),
    };

    match result {
        Ok(()) => process::exit(EXIT_CODE_SUCCESS),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(EXIT_CODE_FAILURE);
        }
    }
}
