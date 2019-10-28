//!
//! The interpreter binary.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use interpreter::Interpreter;
use parser::Parser;

#[derive(Debug, StructOpt)]
#[structopt(name = "zrsi", about = "The ZRust interpreter")]
struct Arguments {
    #[structopt(short = "m", long = "meta", help = "Generates meta info")]
    meta: bool,
    #[structopt(
        short = "i",
        long = "input",
        name = "INPUT",
        parse(from_os_str),
        help = "Specifies the input *.zrs file name"
    )]
    input: PathBuf,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Input: {}", _0)]
    Input(InputError),
    #[fail(display = "Parser: {}", _0)]
    Parser(parser::Error),
    #[fail(display = "Interpreter: {}", _0)]
    Interpreter(interpreter::Error),
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

    let circuit = Parser::default().parse(input).map_err(|error| {
        log::error!("{}", error);
        Error::Parser(error)
    })?;

    if args.meta {
        let meta = serde_json::to_string(&circuit).expect("Always valid");
        log::info!("{}", meta);
    }

    Interpreter::default().interpret(circuit).map_err(|error| {
        log::error!("{}", error);
        Error::Interpreter(error)
    })?;

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "compiler=info,zrsi=info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}
