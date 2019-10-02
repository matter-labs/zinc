//!
//! The Jabberwocky compiler binary.
//!

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jabc", about = "The Jabberwocky language compiler")]
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
        help = "Specifies the input *.jab file name"
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

    log::info!("Input   : {:?}", args.input);
    log::info!("Output  : {:?}", args.output);

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

    let circuit = compiler::parse(code).map_err(|error| {
        log::error!("{}", error);
        Error::Compiler(error)
    })?;

    if args.profile {
        unimplemented!();
    }

    if args.meta {
        println!("{}", serde_json::to_string(&circuit).expect("Always valid"));
    }

    compiler::generate(circuit, args.output).map_err(|error| {
        log::error!("{}", error);
        Error::Compiler(error)
    })?;

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "compiler=info,jabc=info");
    }
    env_logger::Builder::from_default_env()
        .default_format_timestamp_nanos(true)
        .init();
}
