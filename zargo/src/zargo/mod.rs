//!
//! The Zargo package manager binary.
//!

pub(crate) mod arguments;

use std::process;

use self::arguments::Arguments;

///
/// The application entry point.
///
#[tokio::main]
async fn main() {
    let args = Arguments::new();

    zinc_logger::initialize(zinc_const::app_name::ZARGO, args.verbosity, args.quiet);

    process::exit(match args.command.execute().await {
        Ok(()) => zinc_const::exit_code::SUCCESS,
        Err(error) => {
            log::error!("{:?}", error);
            zinc_const::exit_code::FAILURE
        }
    })
}
