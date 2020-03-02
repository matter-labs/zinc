use crate::{Error, IoToError};
use franklin_crypto::bellman::groth16::Parameters;
use pairing::bn256::Bn256;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;

#[derive(Debug, StructOpt)]
#[structopt(name = "prove", about = "Executes circuit and prints program's output")]
pub struct ProveCommand {
    #[structopt(short = "c", long = "circuit", help = "Compiled circuit program file")]
    pub circuit_path: PathBuf,

    #[structopt(short = "k", long = "proving-key", help = "Proving key file")]
    pub proving_key_path: PathBuf,

    #[structopt(short = "w", long = "witness", help = "File with witness values")]
    pub witness_path: PathBuf,

    #[structopt(short = "p", long = "public-data", help = "File with witness values")]
    pub pubdata_path: PathBuf,
}

impl ProveCommand {
    pub fn execute(&self) -> Result<(), Error> {
        // Read program
        let bytes =
            fs::read(&self.circuit_path).error_with_path(|| self.circuit_path.to_string_lossy())?;
        let program = Program::from_bytes(bytes.as_slice()).map_err(Error::ProgramDecoding)?;

        // Read verifying key
        let file = fs::File::open(&self.proving_key_path)
            .error_with_path(|| self.proving_key_path.to_string_lossy())?;
        let params = Parameters::<Bn256>::read(file, true)
            .error_with_path(|| self.proving_key_path.to_string_lossy())?;

        // Read witness
        let witness_json = fs::read_to_string(&self.witness_path)
            .error_with_path(|| self.witness_path.to_string_lossy())?;
        let witness_value = serde_json::from_str(&witness_json)?;
        let witness_struct = Value::from_typed_json(&witness_value, &program.input)?;

        let (pubdata, proof) = zinc_vm::prove::<Bn256>(&program, &params, &witness_struct)?;

        // Write pubdata
        let pubdata_json = serde_json::to_string_pretty(&pubdata.to_json())? + "\n";
        fs::write(&self.pubdata_path, &pubdata_json)
            .error_with_path(|| self.pubdata_path.to_string_lossy())?;

        // Write proof to stdout
        let mut proof_bytes = Vec::new();
        proof.write(&mut proof_bytes).expect("writing to vec");

        let proof_hex = hex::encode(proof_bytes);
        println!("{}", proof_hex);

        Ok(())
    }
}
