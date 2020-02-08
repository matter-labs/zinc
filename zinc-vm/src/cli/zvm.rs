mod commands;

use crate::commands::{Arguments, Command};
use failure::Fail;
use log::LevelFilter;
use std::fmt::Debug;
use std::io;
use std::process::exit;
use structopt::StructOpt;
use zinc_bytecode::data::values::JsonValueError;
use zinc_vm::{RuntimeError, VerificationError};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "io error: {:#?}", _0)]
    IO(io::Error),

    #[fail(display = "runtime error: {:#?}", _0)]
    Runtime(RuntimeError),

    #[fail(display = "failed to verify")]
    Verification(VerificationError),

    #[fail(display = "failed to parse json: {}", _0)]
    JsonDecoding(serde_json::Error),

    #[fail(
        display = "invalid json structure: {}\nNote: remove the file ./data/witness.json so the compiler may recreate it",
        _0
    )]
    JsonValue(JsonValueError),
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
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
        Error::JsonDecoding(error)
    }
}

impl From<JsonValueError> for Error {
    fn from(error: JsonValueError) -> Self {
        Error::JsonValue(error)
    }
}

fn main() {
    let args = Arguments::from_args();

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(match args.verbosity {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .init();

    let result = match args.command {
        Command::Run(command) => command.execute(),
        Command::Setup(command) => command.execute(),
        Command::Prove(command) => command.execute(),
        Command::Verify(command) => command.execute(),
    };

    if let Err(error) = result {
        println!("{}", error);
        exit(1);
    }
}
