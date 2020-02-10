//!
//! The Zinc tester binary.
//!

use std::fs::File;
use std::io::Read;
use std::process;

use failure::Fail;
use log::LevelFilter;
use serde_derive::Deserialize;
use serde_json::Value as JsonValue;
use structopt::StructOpt;

use pairing::bn256::Bn256;

use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

#[derive(Debug, StructOpt)]
#[structopt(name = "znc", about = "The Zinc compiler")]
struct Arguments {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,
}

#[derive(Debug, Deserialize, PartialEq)]
struct TestData {
    input: JsonValue,
    output: JsonValue,
}

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "unimplemented")]
    Unimplemented,
}

fn main() {
    let args: Arguments = Arguments::from_args();

    process::exit(match main_inner(args) {
        Ok(()) => EXIT_CODE_SUCCESS,
        Err(error) => {
            log::error!("{}", error);
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner(args: Arguments) -> Result<(), Error> {
    init_logger(args.verbosity);

    let mut file = File::open("zinc-tester/tests/test.zn").unwrap();
    let size = file.metadata().unwrap().len() as usize;
    let mut string = String::with_capacity(size);
    file.read_to_string(&mut string).unwrap();
    log::debug!("{}", string);

    let json = string
        .lines()
        .filter_map(|line| {
            if line.starts_with("//#") {
                Some(&line[3..])
            } else {
                None
            }
        })
        .collect::<Vec<&str>>()
        .join("");
    log::debug!("{}", json);

    let test_data: TestData = serde_json::from_str(&json).unwrap();

    let bytecode = zinc_compiler::compile_test(string).unwrap();
    let bytecode: Vec<u8> = bytecode.into();
    let program = Program::from_bytes(bytecode.as_slice()).unwrap();
    let input = Value::from_typed_json(&test_data.input, &program.input).unwrap();

    let output = zinc_vm::run::<Bn256>(&program, &input).unwrap();
    let output_json = output.to_json();

    assert_eq!(test_data.output, output_json);

    Ok(())
}

fn init_logger(verbosity: usize) {
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(match verbosity {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .init();
}
