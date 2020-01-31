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
    #[structopt(short = "c", long = "circuit", help = "Compiled circuit program file")]
    pub circuit_path: PathBuf,

    #[structopt(short = "k", long = "proving-key", help = "Proving key file")]
    pub proving_key_path: PathBuf,

    #[structopt(short = "w", long = "witness", help = "File with witness values")]
    pub witness_path: PathBuf,

    #[structopt(short = "p", long = "pubdata", help = "File with witness values")]
    pub pubdata_path: PathBuf,
}

impl ProveCommand {
    pub fn execute(&self) -> Result<(), Error> {
        // Read program
        let bytes = fs::read(&self.circuit_path)?;
        let program = Program::from_bytes(bytes.as_slice()).unwrap();

        // Read verifying key
        let file = fs::File::open(&self.proving_key_path)?;
        let params = Parameters::<Bn256>::read(file, true)?;

        // Read witness
        let witness_json = fs::read_to_string(&self.witness_path)?;
        let witness_struct: Value = serde_json::from_str(&witness_json)?;
        let witness = witness_struct.to_flat_values();

        let (pubdata, proof) = zinc_vm::prove::<Bn256>(&program, &params, witness.as_slice())?;

        // Write pubdata
        let pubdata_struct = Value::from_flat_values(&program.output, &pubdata).unwrap();
        let pubdata_json = serde_json::to_string_pretty(&pubdata_struct)? + "\n";
        fs::write(&self.pubdata_path, &pubdata_json)?;

        // Write proof to stdout
        let mut proof_bytes = Vec::new();
        proof.write(&mut proof_bytes)?;
        let proof_hex = hex::encode(proof_bytes);
        println!("{}", proof_hex);

        Ok(())
    }
}
