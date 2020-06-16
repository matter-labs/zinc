//!
//! The Zinc virtual machine binary `prove` command.
//!

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;

use zinc_vm::IFacade;

use crate::error::Error;
use crate::error::IoToError;

#[derive(Debug, StructOpt)]
#[structopt(name = "prove", about = "Executes circuit and prints program's output")]
pub struct ProveCommand {
    #[structopt(long = "binary", help = "The bytecode file")]
    pub binary_path: PathBuf,

    #[structopt(long = "proving-key", help = "The proving key file")]
    pub proving_key_path: PathBuf,

    #[structopt(long = "witness", help = "The witness JSON file")]
    pub witness_path: PathBuf,

    #[structopt(long = "public-data", help = "The public data JSON file")]
    pub public_data_path: PathBuf,
}

impl ProveCommand {
    pub fn execute(self) -> Result<(), Error> {
        // Read program
        let bytes =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let program =
            BytecodeProgram::from_bytes(bytes.as_slice()).map_err(Error::ProgramDecoding)?;

        // Read verifying key
        let file = fs::File::open(&self.proving_key_path)
            .error_with_path(|| self.proving_key_path.to_string_lossy())?;
        let params = Parameters::<Bn256>::read(file, true)
            .error_with_path(|| self.proving_key_path.to_string_lossy())?;

        // Read witness
        let witness_json = fs::read_to_string(&self.witness_path)
            .error_with_path(|| self.witness_path.to_string_lossy())?;
        let witness_value = serde_json::from_str(&witness_json)?;
        let witness_struct = TemplateValue::from_typed_json(&witness_value, &program.input())?;

        let (pubdata, proof) = program.prove::<Bn256>(params, witness_struct)?;

        // Write pubdata
        let pubdata_json = serde_json::to_string_pretty(&pubdata.to_json())? + "\n";
        fs::write(&self.public_data_path, &pubdata_json)
            .error_with_path(|| self.public_data_path.to_string_lossy())?;

        // Write proof to stdout
        let mut proof_bytes = Vec::new();
        proof.write(&mut proof_bytes).expect("writing to vec");

        let proof_hex = hex::encode(proof_bytes);
        println!("{}", proof_hex);

        Ok(())
    }
}
