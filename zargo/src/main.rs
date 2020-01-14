//!
//! The Zargo package manager binary.
//!

mod command;

pub use self::command::Command;
pub use self::command::Error as CommandError;

use std::env;

use failure::Fail;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "zargo", about = "The Zargo package manager")]
struct Arguments {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "{}", _0)]
    Command(CommandError),
}

fn main() -> Result<(), Error> {
    init_logger();

    let args: Arguments = Arguments::from_args();

    args.command.execute().map_err(Error::Command)?;

    Ok(())
}

fn init_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .init();
}
