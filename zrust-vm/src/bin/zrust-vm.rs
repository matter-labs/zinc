use std::path::PathBuf;
use structopt::StructOpt;
use std::{fs, io};
use franklin_crypto::circuit::test::TestConstraintSystem;
use zrust_vm::{VirtualMachine, RuntimeError, decode_all_vm_instructions, ConstrainedElement, ConstrainedElementOperator};
use bellman::pairing::bn256::Bn256;
use zrust_bytecode::DecodingError;

#[derive(Debug, StructOpt)]
#[structopt(name = "zrustvm", about = "zrust virtual machine")]
struct Arguments {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[derive(Debug)]
enum Error {
    IOError(io::Error),
    DecodingError(DecodingError),
    RuntimeError(RuntimeError),
}

fn main() -> Result<(), Error> {
    env_logger::init();
    
    let args = Arguments::from_args();
    let bytes = fs::read(args.input)
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
