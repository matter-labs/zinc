//!
//! The Zargo circuit manager binary.
//!

mod command;
mod constants;
mod manifest;
mod templates;

pub use self::command::Command;
pub use self::command::Error as CommandError;

use std::env;
use std::process;

use structopt::StructOpt;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

#[derive(Debug, StructOpt)]
#[structopt(name = "zargo", about = "Zinc's circuit manager")]
struct Arguments {
    #[structopt(subcommand)]
    command: Command,
}

fn main() {
    init_logger();

    process::exit(match main_inner() {
        Ok(()) => EXIT_CODE_SUCCESS,
        Err(error) => {
            log::error!("{}", error);
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner() -> Result<(), CommandError> {
    let args = Arguments::from_args();
    args.command.execute()
}

fn init_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .init();
}
