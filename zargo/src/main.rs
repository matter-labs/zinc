//!
//! The Zargo package manager binary.
//!

pub(crate) mod arguments;
pub(crate) mod error;
pub(crate) mod executable;
pub(crate) mod network;
pub(crate) mod project;
pub(crate) mod transaction;

use std::process;

use self::arguments::Arguments;

///
/// The application entry point.
///
#[tokio::main]
async fn main() {
    let args = Arguments::new();

    zinc_logger::initialize(zinc_const::app_name::ZARGO, args.verbosity);

    process::exit(match args.command.execute().await {
        Ok(()) => zinc_const::exit_code::SUCCESS,
        Err(error) => {
            log::error!("{}", error);
            zinc_const::exit_code::FAILURE
        }
    })
}
