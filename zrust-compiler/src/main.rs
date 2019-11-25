//!
//! The parser binary.
//!

#![allow(clippy::large_enum_variant)]

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use zrust_bytecode::dispatch_instruction;
use zrust_bytecode::Instruction;
use zrust_bytecode::InstructionInfo;

#[derive(Debug, StructOpt)]
#[structopt(name = "zrustc", about = "The ZRust compiler")]
struct Arguments {
    #[structopt(
        short = "i",
        long = "input",
        name = "INPUT",
        parse(from_os_str),
        help = "Specifies the *.zrs input file name"
    )]
    input: PathBuf,
    #[structopt(
        short = "o",
        long = "output",
        name = "OUTPUT",
        parse(from_os_str),
        help = "Specifies the *.zrsb output file name"
    )]
    output: PathBuf,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Input: {}", _0)]
    Input(InputError),
    #[fail(display = "Parser: {}", _0)]
    Compiler(zrust_compiler::Error),
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

    let circuit = zrust_compiler::Parser::default()
        .parse(input)
        .map_err(|error| {
            log::error!("{}", error);
            Error::Compiler(error)
        })?;

    log::info!("Output: {:?}", args.output);
    File::create(&args.output)
        .map_err(OutputError::Creating)
        .map_err(Error::Output)?
        .write_all(
            zrust_compiler::Analyzer::default()
                .compile(circuit)
                .map(|instructions| {
                    instructions
                        .into_iter()
                        .enumerate()
                        .map(|(index, instruction)| {
                            log::debug!(">>> {:03} {:?}", index, instruction);
                            dispatch_instruction!(instruction => instruction.encode())
                        })
                        .flatten()
                        .collect::<Vec<u8>>()
                })
                .map_err(|error| {
                    log::error!("{}", error);
                    Error::Compiler(error)
                })?
                .as_slice(),
        )
        .map_err(OutputError::Writing)
        .map_err(Error::Output)?;

    Ok(())
}

fn init_logger() {
    use std::env;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "zrust_compiler=info,zrustc=info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_nanos()
        .init();
}
