mod arguments;

use std::{io, fs};
use zrust_bytecode::{DecodingError, decode_all_instructions};
use bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;
use num_bigint::BigInt;
use std::str::FromStr;
use zrustm::RuntimeError;

#[derive(Debug)]
enum Error {
    IOError(io::Error),
    DecodingError(DecodingError),
    RuntimeError(RuntimeError),
}

struct ExecArguments {
    circuit_file: String,
    witness: Vec<BigInt>,
}

struct GenKeyArguments;
struct GenProofArguments;
struct VerifyArguments;

enum Arguments {
    Exec(ExecArguments),
    GenKey(GenKeyArguments),
    GenProof(GenProofArguments),
    Verify(VerifyArguments),
    Empty
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let args = parse_arguments();

    match args {
        Arguments::Exec(args) => exec(args)?,
        Arguments::GenKey(args) => unimplemented!(),
        Arguments::GenProof(_args) => unimplemented!(),
        Arguments::Verify(_args) => unimplemented!(),
        Arguments::Empty => {},
    }

    Ok(())
}

fn exec(args: ExecArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file)
        .map_err(|e| Error::IOError(e))?;

    let mut code = decode_all_instructions(bytes.as_slice())
        .map_err(|e| Error::DecodingError(e))?;

    let outputs = zrustm::exec::<Bn256>(code.as_slice(), args.witness.as_slice())
        .map_err(|e| Error::RuntimeError(e))?;

    for value in outputs.iter() {
        match value {
            None => println!("None"),
            Some(value) => println!("{}", value),
        }
    }

    Ok(())
}

//fn gen_key(args: GenKeyArguments) -> Result<(), Error> {
//    let bytes = fs::read(args.circuit_file)
//        .map_err(|e| Error::IOError(e))?;
//
//    let mut code = decode_all_instructions(bytes.as_slice())
//        .map_err(|e| Error::DecodingError(e))?;
//
//    let key = zrustm::gen_key::<Bn256>(code.as_slice())
//        .map_err(|e| Error::RuntimeError(e))?;
//
//    Ok(())
//}

fn parse_arguments() -> Arguments {
    let args = arguments::build_arguments().get_matches();

    match args.subcommand() {
        ("exec", Some(command_args)) => {
            let circuit_file = command_args.value_of("circuit").expect("--circuit is required");
            let witness = {
                command_args
                    .values_of("witness")
                    .unwrap()
                    .into_iter()
                    .map(|s| BigInt::from_str(s).unwrap())
                    .collect()
            };
            Arguments::Exec(ExecArguments {
                circuit_file: circuit_file.into(),
                witness,
            })
        },
        ("gen-key", Some(_command_args)) => {
            Arguments::GenKey(GenKeyArguments)
        },
        ("gen-proof", Some(_command_args)) => {
            Arguments::GenProof(GenProofArguments)
        },
        ("verify", Some(_command_args)) => {
            Arguments::Verify(VerifyArguments)
        },
        ("", _) => {
            Arguments::Empty
        }
        (command, _) => {
            panic!("Unknown command {:?}", command)
        }
    }
}
