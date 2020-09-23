//!
//! The Zargo project manager binary.
//!

pub(crate) mod arguments;
pub(crate) mod error;
pub(crate) mod executable;
pub(crate) mod project;
pub(crate) mod transfer;

use std::process;

use self::arguments::command::IExecutable;
use self::arguments::Arguments;

///
/// The application entry point.
///
fn main() {
    let args = Arguments::new();

    zinc_utils::initialize_logger(zinc_const::app_name::ZARGO, args.verbosity);

    process::exit(match args.command.execute() {
        Ok(()) => zinc_const::exit_code::SUCCESS,
        Err(error) => {
            log::error!("{}", error);
            zinc_const::exit_code::FAILURE
        }
    })
}
