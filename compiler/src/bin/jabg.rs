//!
//! The Jabberwocky generator binary.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jabg", about = "The Jabberwocky language generator")]
struct Arguments {
    #[structopt(name = "INPUT", parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug, Fail)]
#[allow(clippy::large_enum_variant)]
enum Error {
    #[fail(display = "Input: {}", _0)]
    Input(InputError),
    #[fail(display = "Parsing: {}", _0)]
    Parsing(compiler::Error),
    #[fail(display = "Generating: {}", _0)]
    Generating(compiler::Error),
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

    let mut file = File::open(&args.input)
        .map_err(InputError::Opening)
        .map_err(Error::Input)?;
    let size = file
        .metadata()
        .map_err(InputError::Metadata)
        .map_err(Error::Input)?
        .len() as usize;
    let mut code = String::with_capacity(size);
    file.read_to_string(&mut code)
        .map_err(InputError::Reading)
        .map_err(Error::Input)?;

    let program = compiler::parse(code).map_err(|error| {
        log::error!("{}", error);
        Error::Parsing(error)
    })?;
    compiler::generate(program).map_err(|error| {
        log::error!("{}", error);
        Error::Generating(error)
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
