//!
//! The Jabberwocky interpreter binary.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jabi", about = "The Jabberwocky language interpreter")]
struct Arguments {
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, Fail)]
#[allow(clippy::large_enum_variant)]
enum Error {
    #[fail(display = "Input opening: {}", _0)]
    InputOpening(std::io::Error),
    #[fail(display = "Input metadata: {}", _0)]
    InputMetadata(std::io::Error),
    #[fail(display = "Input reading: {}", _0)]
    InputReading(std::io::Error),
    #[fail(display = "Parsing: {}", _0)]
    Parsing(compiler::Error),
    #[fail(display = "Interpreting: {}", _0)]
    Interpreting(compiler::Error),
}

fn main() -> Result<(), Error> {
    init_logger();

    let args: Arguments = Arguments::from_args();

    let mut file = File::open(&args.input).map_err(Error::InputOpening)?;
    let size = file.metadata().map_err(Error::InputMetadata)?.len() as usize;
    let mut code = String::with_capacity(size);
    file.read_to_string(&mut code)
        .map_err(Error::InputReading)?;

    let program = compiler::parse(code).map_err(|error| {
        log::error!("{}", error);
        Error::Parsing(error)
    })?;
    compiler::interpret(program).map_err(|error| {
        log::error!("{}", error);
        Error::Interpreting(error)
    })?;

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "compiler=info,jabi=info");
    }
    env_logger::Builder::from_default_env()
        .default_format_timestamp_nanos(true)
        .init();
}
