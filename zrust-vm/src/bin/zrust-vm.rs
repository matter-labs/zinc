use std::path::PathBuf;
use structopt::StructOpt;
use std::fs::File;
use std::io::Read;
use franklin_crypto::circuit::test::TestConstraintSystem;
use zrust_vm::{VirtualMachine, Bytecode, RuntimeError};
use bellman::pairing::bn256::{Bn256, Fr};

#[derive(Debug, StructOpt)]
#[structopt(name = "zrustvm", about = "zrust virtual machine")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let mut file = File::open(&opt.input).expect("failed to open file");
    let size = file.metadata().expect("failed to get file metadata").len() as usize;
    let mut input: Vec<u8> = Vec::new();
    file.read_to_end(&mut input).expect("failed to read file");
}

fn execute(bytes: &[u8]) -> Result<Fr, RuntimeError> {
    let mut cs = TestConstraintSystem::<Bn256>::new();
    let mut vm = VirtualMachine::<Bn256, TestConstraintSystem<Bn256>>::new();
    let mut bytecode = Bytecode::new(bytes);

    vm.run(&mut cs, &mut bytecode)?;

    let top = vm
        .stack()
        .top()
        .ok_or(RuntimeError::StackUnderflow)?
        .value
        .ok_or(RuntimeError::SynthesisError)?;

    Ok(top)
}
