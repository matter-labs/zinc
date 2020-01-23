mod commands;

use crate::commands::{Arguments, Command};
use log::LevelFilter;
use std::fmt::Debug;
use std::io;
use structopt::StructOpt;
use zinc_bytecode::DecodingError;
use zinc_vm::{RuntimeError, VerificationError};

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Decoding(DecodingError),
    Runtime(RuntimeError),
    Verification(VerificationError),
    Json(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<DecodingError> for Error {
    fn from(error: DecodingError) -> Self {
        Error::Decoding(error)
    }
}

impl From<RuntimeError> for Error {
    fn from(error: RuntimeError) -> Self {
        Error::Runtime(error)
    }
}

impl From<VerificationError> for Error {
    fn from(error: VerificationError) -> Self {
        Error::Verification(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

fn main() -> Result<(), Error> {
    let args = Arguments::from_args();

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(if args.verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Warn
        })
        .init();

    match args.command {
        Command::Exec(command) => command.execute(),
        Command::Setup(command) => command.execute(),
        Command::Prove(command) => command.execute(),
        Command::Verify(command) => command.execute(),
    }?;

    Ok(())
}
