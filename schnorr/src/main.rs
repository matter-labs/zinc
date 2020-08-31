//!
//! The Schnorr signature tool binary.
//!

mod arguments;
mod error;

use std::process;

use self::arguments::command::IExecutable;
use self::arguments::Arguments;

///
/// The application entry point.
///
fn main() {
    let args = Arguments::new();

    zinc_utils::initialize_logger(zinc_const::app_name::SCHNORR, args.verbosity);

    match args.command.execute() {
        Ok(()) => process::exit(zinc_const::exit_code::SUCCESS),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(zinc_const::exit_code::FAILURE);
        }
    }
}
