use crate::Error;
use franklin_crypto::bellman::groth16::Parameters;
use pairing::bn256::Bn256;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;

#[derive(Debug, StructOpt)]
pub struct ProveCommand {
    #[structopt(short = "c", long = "circuit", about = "Circuit's bytecode file")]
    pub circuit_path: PathBuf,

    #[structopt(
        short = "P",
        long = "params",
        about = "Generated parameters file for prover and verifier"
    )]
    pub params_path: PathBuf,

    #[structopt(short = "i", long = "input", about = "Program's input file")]
    pub input_path: PathBuf,

    #[structopt(short = "p", long = "proof", about = "Proof file to write")]
    pub proof_path: PathBuf,
}

impl ProveCommand {
    pub fn execute(&self) -> Result<(), Error> {
        // Read & parse bytecode
        let bytes = fs::read(&self.circuit_path)?;
        let program = Program::from_bytes(bytes.as_slice()).unwrap();

        // Read & parse params
        let file = fs::File::open(&self.params_path)?;
        let params = Parameters::<Bn256>::read(file, true)?;

        let input_text = fs::read_to_string(&self.input_path)?;
        let input_values: Value = serde_json::from_str(&input_text)?;
        let input = input_values.to_flat_values();

        let proof = zinc_vm::prove::<Bn256>(&program, &params, input.as_slice())?;

        // Write proof
        let file = fs::File::create(&self.proof_path)?;
        proof.write(file).map_err(Error::IO)?;

        Ok(())
    }
}
