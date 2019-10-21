//!
//! The transpiler binary.
//!

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use transpiler::Transpiler;

#[derive(Debug, StructOpt)]
#[structopt(name = "zrsc", about = "The ZRust transpiler")]
struct Arguments {
    #[structopt(
        short = "p",
        long = "profile",
        help = "Runs the profiler and prints cost information"
    )]
    profile: bool,
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
    #[structopt(
        short = "o",
        long = "output",
        name = "OUTPUT",
        parse(from_os_str),
        help = "Specifies the output *.rs file name"
    )]
    output: PathBuf,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Input: {}", _0)]
    Input(InputError),
    #[fail(display = "Parser: {}", _0)]
    Parser(parser::Error),
    #[fail(display = "Transpiler: {}", _0)]
    Transpiler(transpiler::Error),
    #[fail(display = "Output: {}", _0)]
    Output(OutputError),
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

#[derive(Debug, Fail)]
enum OutputError {
    #[fail(display = "Creating: {}", _0)]
    Creating(std::io::Error),
    #[fail(display = "Writing: {}", _0)]
    Writing(std::io::Error),
}

fn main() -> Result<(), Error> {
    init_logger();

    let args: Arguments = Arguments::from_args();

    log::info!("Input   : {:?}", args.input);
    log::info!("Output  : {:?}", args.output);

    let mut input_file = File::open(&args.input)
        .map_err(InputError::Opening)
        .map_err(Error::Input)?;
    let size = input_file
        .metadata()
        .map_err(InputError::Metadata)
        .map_err(Error::Input)?
        .len() as usize;
    let mut input = String::with_capacity(size);
    input_file
        .read_to_string(&mut input)
        .map_err(InputError::Reading)
        .map_err(Error::Input)?;

    let circuit = parser::parse(input).map_err(|error| {
        log::error!("{}", error);
        Error::Parser(error)
    })?;

    if args.meta {
        let meta = serde_json::to_string(&circuit).expect("Always valid");
        log::info!("{}", meta);
    }

    let result = Transpiler::default().transpile(circuit).map_err(|error| {
        log::error!("{}", error);
        Error::Transpiler(error)
    })?;

    let mut output_file = File::create(&args.output)
        .map_err(OutputError::Creating)
        .map_err(Error::Output)?;
    output_file
        .write_all(result.as_bytes())
        .map_err(OutputError::Writing)
        .map_err(Error::Output)?;

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "compiler=info,zrsc=info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}
