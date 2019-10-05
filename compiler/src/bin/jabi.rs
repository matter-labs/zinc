//!
//! The Jabberwocky interpreter binary.
//!

#![allow(clippy::large_enum_variant)]

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jabi", about = "The Jabberwocky language interpreter")]
struct Arguments {
    #[structopt(
        short = "i",
        long = "input",
        name = "INPUT",
        parse(from_os_str),
        help = "Specifies the input *.jab file name"
    )]
    input: PathBuf,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Input: {}", _0)]
    Input(InputError),
    #[fail(display = "Compiler: {}", _0)]
    Compiler(compiler::Error),
}

#[derive(Debug, Fail)]
enum InputError {
    #[fail(display = "Opening: {}", _0)]
    Opening(std::io::Error),
    #[fail(display = "Metadata: {}", _0)]
    Metadata(std::io::Error),
    #[fail(display = "Reading: {}", _0)]
    Reading(std::io::Error),
}

fn main() -> Result<(), Error> {
    init_logger();

    let args: Arguments = Arguments::from_args();

    log::info!("Input: {:?}", args.input);

    let mut file = File::open(&args.input)
        .map_err(InputError::Opening)
        .map_err(Error::Input)?;
    let size = file
        .metadata()
        .map_err(InputError::Metadata)
        .map_err(Error::Input)?
        .len() as usize;
    let mut input = String::with_capacity(size);
    file.read_to_string(&mut input)
        .map_err(InputError::Reading)
        .map_err(Error::Input)?;

    let circuit = compiler::parse(input).map_err(|error| {
        log::error!("{}", error);
        Error::Compiler(error)
    })?;

    compiler::interpret(circuit).map_err(|error| {
        log::error!("{}", error);
        Error::Compiler(error)
    })?;

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "compiler=info,jabi=info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}
