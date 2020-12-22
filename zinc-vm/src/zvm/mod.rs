//!
//! The Zinc virtual machine binary.
//!

pub(crate) mod arguments;
pub(crate) mod error;

use std::process;

use self::arguments::command::IExecutable;
use self::arguments::Arguments;

fn main() {
    let args = Arguments::new();

    zinc_logger::initialize(
        zinc_const::app_name::VIRTUAL_MACHINE,
        args.verbosity,
        args.quiet,
    );

    match args.command.execute() {
        Ok(exit_code) => process::exit(exit_code),
        Err(error) => {
            log::error!("{:?}", error);
            process::exit(zinc_const::exit_code::FAILURE);
        }
    }
}
