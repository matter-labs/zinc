mod arguments;

use bellman::pairing::bn256::Bn256;
use log::LevelFilter;
use num_bigint::BigInt;
use std::fmt::Debug;
use std::str::FromStr;
use std::{fs, io};
use zrust_bytecode::{decode_all_instructions, DecodingError};
use zrustm::{RuntimeError, VerificationError};
use franklin_crypto::bellman::groth16::Proof;
use std::process::exit;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    Decoding(DecodingError),
    Runtime(RuntimeError),
    Verification(VerificationError)
}

struct ExecArguments {
    circuit_file: String,
    witness: Vec<BigInt>,
}

struct ProveArguments {
    circuit_file: String,
    witness: Vec<BigInt>,
    output_file: String,
}

struct VerifyArguments {
    circuit_file: String,
    proof_file: String,
    input: Vec<BigInt>,
}

enum CommandArgs {
    Exec(ExecArguments),
    Prove(ProveArguments),
    Verify(VerifyArguments),
}

struct Args {
    command_args: CommandArgs,
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let args = parse_arguments();

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(if args.verbose { LevelFilter::Info } else { LevelFilter::Warn })
        .init();

    match args.command_args {
        CommandArgs::Exec(args) => exec(args)?,
        CommandArgs::Prove(args) => prove(args)?,
        CommandArgs::Verify(args) => verify(args)?,
    }

    Ok(())
}

fn exec(args: ExecArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file).map_err(Error::IO)?;

    let code = decode_all_instructions(bytes.as_slice()).map_err(Error::Decoding)?;

    let outputs =
        zrustm::exec::<Bn256>(code.as_slice(), args.witness.as_slice()).map_err(Error::Runtime)?;

    for value in outputs.iter() {
        match value {
            None => println!("None"),
            Some(value) => println!("{}", value),
        }
    }

    Ok(())
}

fn prove(args: ProveArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file).map_err(Error::IO)?;

    let code = decode_all_instructions(bytes.as_slice()).map_err(Error::Decoding)?;

    let proof =
        zrustm::prove::<Bn256>(code.as_slice(), args.witness.as_slice()).map_err(Error::Runtime)?;

    let file = fs::File::create(args.output_file).map_err(Error::IO)?;
    proof.write(file).map_err(Error::IO)?;

    Ok(())
}

fn verify(args: VerifyArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file)
        .map_err(Error::IO)?;

    let code = decode_all_instructions(bytes.as_slice())
        .map_err(Error::Decoding)?;

    let key = zrustm::gen_key::<Bn256>(code.as_slice())
        .map_err(Error::Runtime)?;

    let file = fs::File::open(args.proof_file).map_err(Error::IO)?;
    let proof = Proof::<Bn256>::read(file).map_err(Error::IO)?;

    let verified = zrustm::verify(&key, &proof, args.input.as_slice()).map_err(Error::Verification)?;

    if verified {
        println!("Ok");
    } else {
        println!("Failed");
        exit(1);
    }

    Ok(())
}

fn parse_arguments() -> Args {
    let args = arguments::build_arguments().get_matches();

    let verbose = args.is_present("verbose");

    let command_args = match args.subcommand() {
        ("exec", Some(command_args)) => {
            let circuit_file = command_args
                .value_of("circuit")
                .expect("--circuit is required");
            let witness = {
                match command_args.values_of("witness") {
                    Some(values) => {
                        values
                            .map(|s| BigInt::from_str(s).unwrap())
                            .collect()
                    },
                    None => Vec::new(),
                }
            };
            CommandArgs::Exec(ExecArguments {
                circuit_file: circuit_file.into(),
                witness,
            })
        }
        ("prove", Some(command_args)) => {
            let circuit_file = command_args
                .value_of("circuit")
                .expect("--circuit is required");
            let output_file = command_args
                .value_of("output")
                .expect("--output is required");
            let witness = {
                match command_args.values_of("witness") {
                    Some(values) => {
                        values
                            .map(|s| BigInt::from_str(s).unwrap())
                            .collect()
                    },
                    None => Vec::new(),
                }
            };
            CommandArgs::Prove(ProveArguments {
                circuit_file: circuit_file.into(),
                witness,
                output_file: output_file.into()
            })
        }
        ("verify", Some(command_args)) => {
            let circuit_file = command_args
                .value_of("circuit")
                .expect("--circuit is required");
            let proof = command_args
                .value_of("proof")
                .expect("--proof is required");
            let input = {
                match command_args.values_of("input") {
                    Some(values) => {
                        values
                            .map(|s| BigInt::from_str(s).unwrap())
                            .collect()
                    },
                    None => Vec::new(),
                }
            };
            CommandArgs::Verify(VerifyArguments {
                circuit_file: circuit_file.to_string(),
                proof_file: proof.to_string(),
                input,
            })
        }
        (command, _) => panic!("Unknown command {:?}", command),
    };

    Args { verbose, command_args }
}
