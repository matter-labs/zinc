//!
//! The Schnorr signature tool binary.
//!

mod arguments;
mod error;

use std::process;

use self::arguments::command::IExecutable;
use self::arguments::Arguments;

fn main() {
    let args = Arguments::new();

    match args.command.execute() {
        Ok(()) => process::exit(zinc_const::exit_code::SUCCESS),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(zinc_const::exit_code::FAILURE);
        }
    }
}
