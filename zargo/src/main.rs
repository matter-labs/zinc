//!
//! The Zargo project manager binary.
//!

//#![deny(missing_docs)]
//#![deny(clippy::missing_docs_in_private_items)]

mod arguments;
mod directory;
mod executable;
mod manifest;

use std::process;

use self::arguments::command::IExecutable;
use self::arguments::Arguments;

fn main() {
    let args = Arguments::new();

    zinc_utils::logger::initialize(zinc_const::app_name::ZARGO, args.verbosity);

    process::exit(match args.command.execute() {
        Ok(()) => zinc_const::exit_code::SUCCESS,
        Err(error) => {
            log::error!("{}", error);
            zinc_const::exit_code::FAILURE
        }
    })
}
