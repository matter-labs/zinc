mod arguments;

use bellman::pairing::bn256::Bn256;
use colored::*;
use franklin_crypto::bellman::groth16::{Parameters, Proof};
use log::LevelFilter;
use num_bigint::BigInt;
use std::fmt::Debug;
use std::process::exit;
use std::str::FromStr;
use std::{fs, io};
use zinc_bytecode::{decode_all_instructions, DecodingError};
use zinc_vm::{RuntimeError, VerificationError};

#[derive(Debug)]
enum Error {
    IO(io::Error),
    Decoding(DecodingError),
    Runtime(RuntimeError),
    Verification(VerificationError),
}

struct ExecArguments {
    circuit_file: String,
    witness: Vec<BigInt>,
}

struct ProveArguments {
    circuit_file: String,
    params_file: String,
    output_file: String,
    witness: Vec<BigInt>,
}

struct VerifyArguments {
    params_file: String,
    proof_file: String,
    public_input: Vec<BigInt>,
}

struct SetupArguments {
    circuit_file: String,
    output_file: String,
}

enum CommandArgs {
    Exec(ExecArguments),
    Prove(ProveArguments),
    Verify(VerifyArguments),
    Setup(SetupArguments),
}

struct Args {
    command_args: CommandArgs,
    verbose: bool,
}

fn main() -> Result<(), Error> {
    let args = parse_arguments();

    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(if args.verbose {
            LevelFilter::Info
        } else {
            LevelFilter::Warn
        })
        .init();

    match args.command_args {
        CommandArgs::Exec(args) => exec(args)?,
        CommandArgs::Prove(args) => prove(args)?,
        CommandArgs::Verify(args) => verify(args)?,
        CommandArgs::Setup(args) => setup(args)?,
    }

    Ok(())
}

fn exec(args: ExecArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file).map_err(Error::IO)?;

    let code = decode_all_instructions(bytes.as_slice()).map_err(Error::Decoding)?;

    let outputs =
        zinc_vm::exec::<Bn256>(code.as_slice(), args.witness.as_slice()).map_err(Error::Runtime)?;

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

    let file = fs::File::open(args.params_file).map_err(Error::IO)?;

    let params = Parameters::<Bn256>::read(file, true).map_err(Error::IO)?;

    let proof = zinc_vm::prove::<Bn256>(code.as_slice(), &params, args.witness.as_slice())
        .map_err(Error::Runtime)?;

    let file = fs::File::create(args.output_file).map_err(Error::IO)?;

    proof.write(file).map_err(Error::IO)?;

    Ok(())
}

fn verify(args: VerifyArguments) -> Result<(), Error> {
    let params_file = fs::File::open(args.params_file).map_err(Error::IO)?;
    let params = Parameters::<Bn256>::read(params_file, true).map_err(Error::IO)?;

    let proof_file = fs::File::open(args.proof_file).map_err(Error::IO)?;
    let proof = Proof::<Bn256>::read(proof_file).map_err(Error::IO)?;

    let verified = zinc_vm::verify(&params, &proof, args.public_input.as_slice())
        .map_err(Error::Verification)?;

    if verified {
        println!("{}", "Ok".bold().green());
    } else {
        println!("{}", "Verification failed".bold().red());
        exit(1);
    }

    Ok(())
}

fn setup(args: SetupArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file).map_err(Error::IO)?;
    let code = decode_all_instructions(bytes.as_slice()).map_err(Error::Decoding)?;
    let params = zinc_vm::setup::<Bn256>(code.as_slice()).map_err(Error::Runtime)?;

    let file = fs::File::create(args.output_file).map_err(Error::IO)?;
    params.write(file).map_err(Error::IO)?;

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
                    Some(values) => values.map(|s| BigInt::from_str(s).unwrap()).collect(),
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
            let params_file = command_args
                .value_of("params")
                .expect("--params is required");
            let witness = {
                match command_args.values_of("witness") {
                    Some(values) => values.map(|s| BigInt::from_str(s).unwrap()).collect(),
                    None => Vec::new(),
                }
            };
            CommandArgs::Prove(ProveArguments {
                circuit_file: circuit_file.into(),
                witness,
                output_file: output_file.into(),
                params_file: params_file.into(),
            })
        }
        ("verify", Some(command_args)) => {
            let params_file = command_args
                .value_of("params")
                .expect("--params is required");
            let proof_file = command_args.value_of("proof").expect("--proof is required");
            let input = {
                match command_args.values_of("public-input") {
                    Some(values) => values.map(|s| BigInt::from_str(s).unwrap()).collect(),
                    None => Vec::new(),
                }
            };
            CommandArgs::Verify(VerifyArguments {
                params_file: params_file.to_string(),
                proof_file: proof_file.to_string(),
                public_input: input,
            })
        }
        ("setup", Some(command_args)) => {
            let circuit_file = command_args
                .value_of("circuit")
                .expect("--circuit is required");
            let output_file = command_args
                .value_of("output")
                .expect("--output is required");
            CommandArgs::Setup(SetupArguments {
                circuit_file: circuit_file.to_string(),
                output_file: output_file.to_string(),
            })
        }
        (command, _) => panic!("Unknown command {:?}", command),
    };

    Args {
        verbose,
        command_args,
    }
}
