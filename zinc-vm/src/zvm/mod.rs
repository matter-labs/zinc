//!
//! The Zinc virtual machine binary.
//!

mod arguments;
mod error;

use std::process;

use self::arguments::command::IExecutable;
use self::arguments::Arguments;

fn main() {
    let args = Arguments::new();

    zinc_utils::logger::initialize(zinc_const::app_name::ZINC_VIRTUAL_MACHINE, args.verbosity);

    match args.command.execute() {
        Ok(exit_code) => process::exit(exit_code),
        Err(error) => {
            eprintln!("{}", error);
            process::exit(zinc_const::exit_code::FAILURE);
        }
    }
}
