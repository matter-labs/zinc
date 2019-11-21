use std::{io, fs};
use zrust_bytecode::DecodingError;
use zrustm::{RuntimeError, cli, VirtualMachine, ConstrainedElementOperator, decode_all_vm_instructions, ConstrainedElement};
use bellman::pairing::bn256::Bn256;
use franklin_crypto::circuit::test::TestConstraintSystem;

#[derive(Debug)]
enum Error {
    IOError(io::Error),
    DecodingError(DecodingError),
    RuntimeError(RuntimeError),
}

struct ExecArguments {
    circuit_file: String
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
        Arguments::GenKey(_args) => unimplemented!(),
        Arguments::GenProof(_args) => unimplemented!(),
        Arguments::Verify(_args) => unimplemented!(),
        Arguments::Empty => {},
    }

    Ok(())
}

fn exec(args: ExecArguments) -> Result<(), Error> {
    let bytes = fs::read(args.circuit_file)
        .map_err(|e| Error::IOError(e))?;

    let cs = TestConstraintSystem::<Bn256>::new();
    let mut vm = VirtualMachine::new(ConstrainedElementOperator::new(cs));

    let mut instructions = decode_all_vm_instructions::<
        ConstrainedElement<Bn256>,
        ConstrainedElementOperator<Bn256, TestConstraintSystem<Bn256>>
    >(bytes.as_slice())
        .map_err(|e| Error::DecodingError(e))?;

    vm
        .run(instructions.as_mut_slice())
        .map_err(|e| Error::RuntimeError(e))?;

    Ok(())
}

fn parse_arguments() -> Arguments {
    let args = cli::build_arguments().get_matches();

    match args.subcommand() {
        ("exec", Some(command_args)) => {
            let circuit_file = command_args.value_of("circuit").expect("--circuit is required");
            Arguments::Exec(ExecArguments { circuit_file: circuit_file.into() })
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
