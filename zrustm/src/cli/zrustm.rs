mod arguments;

use bellman::pairing::bn256::Bn256;
use log::LevelFilter;
use num_bigint::BigInt;
use std::fmt::Debug;
use std::str::FromStr;
use std::{fs, io};
use zrust_bytecode::{decode_all_instructions, DecodingError};
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

struct GenKeyArguments {
    circuit_file: String,
}

struct GenProofArguments {
    circuit_file: String,
    witness: Vec<BigInt>,
}

struct VerifyArguments {
    circuit_file: String,
    proof: String,
}

enum Arguments {
    Exec(ExecArguments),
    GenKey(GenKeyArguments),
    GenProof(GenProofArguments),
    Verify(VerifyArguments),
    Empty,
}

fn main() -> Result<(), Error> {
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(LevelFilter::Info)
        .init();

    let args = parse_arguments();

    match args {
        Arguments::Exec(args) => exec(args)?,
        Arguments::GenKey(args) => gen_key(args)?,
        Arguments::GenProof(args) => gen_proof(args)?,
        Arguments::Verify(args) => verify(args)?,
        Arguments::Empty => {}
    }

    Ok(())
}

fn exec(args: ExecArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file).map_err(|e| Error::IOError(e))?;

    let code = decode_all_instructions(bytes.as_slice()).map_err(|e| Error::DecodingError(e))?;

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

fn gen_key(args: GenKeyArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file).map_err(|e| Error::IOError(e))?;

    let code = decode_all_instructions(bytes.as_slice()).map_err(|e| Error::DecodingError(e))?;

    let _key = zrustm::gen_key::<Bn256>(code.as_slice()).map_err(|e| Error::RuntimeError(e))?;

    println!("Successfully generated key");
    //    dbg!(key);

    Ok(())
}

fn gen_proof(args: GenProofArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file).map_err(|e| Error::IOError(e))?;

    let code = decode_all_instructions(bytes.as_slice()).map_err(|e| Error::DecodingError(e))?;

    let proof = zrustm::prove::<Bn256>(code.as_slice(), args.witness.as_slice())
        .map_err(|e| Error::RuntimeError(e))?;

    dbg!(proof);

    Ok(())
}

fn verify(_args: VerifyArguments) -> Result<(), Error> {
    //    let bytes = fs::read(args.circuit_file)
    //        .map_err(|e| Error::IOError(e))?;
    //
    //    let mut code = decode_all_instructions(bytes.as_slice())
    //        .map_err(|e| Error::DecodingError(e))?;
    //
    //    let key = zrustm::gen_key::<Bn256>(code.as_slice())
    //        .map_err(|e| Error::RuntimeError(e))?;
    //
    //    zrustm::verify(key, )
    Ok(())
}

fn parse_arguments() -> Arguments {
    let args = arguments::build_arguments().get_matches();

    match args.subcommand() {
        ("exec", Some(command_args)) => {
            let circuit_file = command_args
                .value_of("circuit")
                .expect("--circuit is required");
            let witness = {
                command_args
                    .values_of("witness")
                    .expect("--witness is required")
                    .into_iter()
                    .map(|s| BigInt::from_str(s).unwrap())
                    .collect()
            };
            Arguments::Exec(ExecArguments {
                circuit_file: circuit_file.into(),
                witness,
            })
        }
        ("gen-key", Some(command_args)) => {
            let circuit_file = command_args
                .value_of("circuit")
                .expect("--circuit is required");
            Arguments::GenKey(GenKeyArguments {
                circuit_file: circuit_file.into(),
            })
        }
        ("prove", Some(command_args)) => {
            let circuit_file = command_args
                .value_of("circuit")
                .expect("--circuit is required");
            let witness = {
                command_args
                    .values_of("witness")
                    .expect("--witness is required")
                    .into_iter()
                    .map(|s| BigInt::from_str(s).unwrap())
                    .collect()
            };
            Arguments::GenProof(GenProofArguments {
                circuit_file: circuit_file.into(),
                witness,
            })
        }
        ("verify", Some(command_args)) => {
            let circuit_file = command_args
                .value_of("circuit")
                .expect("--circuit is required");
            let proof = command_args.value_of("proof").expect("--proof is required");
            Arguments::Verify(VerifyArguments {
                circuit_file: circuit_file.to_string(),
                proof: proof.to_string(),
            })
        }
        ("", _) => Arguments::Empty,
        (command, _) => panic!("Unknown command {:?}", command),
    }
}
