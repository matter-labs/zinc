//!
//! The parser binary.
//!

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use failure::Fail;
use structopt::StructOpt;

use zinc_bytecode::dispatch_instruction;
use zinc_bytecode::Instruction;
use zinc_bytecode::InstructionInfo;

#[derive(Debug, StructOpt)]
#[structopt(name = "znc", about = "The Zinc compiler")]
struct Arguments {
    #[structopt(
        short = "i",
        long = "input",
        name = "INPUT",
        parse(from_os_str),
        help = "Specifies the *.zn input file name"
    )]
    input: PathBuf,
    #[structopt(
        short = "o",
        long = "output",
        name = "OUTPUT",
        parse(from_os_str),
        help = "Specifies the *.znb output file name"
    )]
    output: PathBuf,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "Input: {}", _0)]
    Input(InputError),
    #[fail(display = "Parser: {}", _0)]
    Compiler(zinc_compiler::Error),
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
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .init();

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

    let circuit = zinc_compiler::Parser::default()
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
            zinc_compiler::BinaryAnalyzer::default()
                .compile(circuit)
                .map(|instructions| {
                    instructions
                        .into_iter()
                        .enumerate()
                        .map(|(index, instruction)| {
                            log::trace!("{:03} {:?}", index, instruction);
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
