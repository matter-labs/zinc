//!
//! The Zinc virtual machine `verify` subcommand.
//!

use std::fs;
use std::io::Read;
use std::path::PathBuf;

use colored::Colorize;
use structopt::StructOpt;

use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::groth16::VerifyingKey;
use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_build::Program as BuildProgram;
use zinc_build::Value as BuildValue;

use zinc_vm::Facade;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `verify` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "verify", about = "Verifies the proof using verifying key")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary", help = "The bytecode file")]
    pub binary_path: PathBuf,

    /// The path to the verifying key file.
    #[structopt(long = "verifying-key", help = "The verifying key path")]
    pub verifying_key_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(long = "public-data", help = "Path to public data JSON file")]
    pub public_data_path: PathBuf,

    /// The method name to call, if the program is a contract.
    #[structopt(long = "method", help = "The method name")]
    pub method: Option<String>,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        // Read proof
        let mut proof = String::new();
        std::io::stdin()
            .read_to_string(&mut proof)
            .error_with_path(|| "<stdin>")?;
        let proof = hex::decode(proof.trim()).map_err(|error| Error::HexDecoding {
            context: "proof".to_owned(),
            error,
        })?;
        let proof = Proof::<Bn256>::read(proof.as_slice()).error_with_path(|| "<proof data>")?;

        // Read program
        let bytes =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let program =
            BuildProgram::try_from_slice(bytes.as_slice()).map_err(Error::ProgramDecoding)?;

        // Read verification key
        let mut verifying_key_file = fs::File::open(&self.verifying_key_path)
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;
        let mut verifying_key = Vec::new();
        verifying_key_file
            .read_to_end(&mut verifying_key)
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;
        let verifying_key = VerifyingKey::<Bn256>::read(verifying_key.as_slice())
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;

        // Read public input
        let output_text = fs::read_to_string(&self.public_data_path)
            .error_with_path(|| self.public_data_path.to_string_lossy())?;
        let output_json = serde_json::from_str(output_text.as_str())?;
        let output_type = match program {
            BuildProgram::Circuit(circuit) => circuit.output,
            BuildProgram::Contract(contract) => {
                let method_name = self.method.ok_or(Error::MethodNameNotFound)?;
                let method = contract
                    .methods
                    .get(method_name.as_str())
                    .cloned()
                    .ok_or(Error::MethodNotFound { name: method_name })?;
                if method.is_mutable {
                    method.output.into_mutable_method_output()
                } else {
                    method.output
                }
            }
        };
        let output_value = BuildValue::try_from_typed_json(output_json, output_type)?;

        // Verify
        let verified = Facade::verify::<Bn256>(verifying_key, proof, output_value)?;

        Ok(if verified {
            println!("{}", " ✔ Verified".bold().green());
            zinc_const::exit_code::SUCCESS as i32
        } else {
            println!("{}", " ✘   Failed".bold().red());
            zinc_const::exit_code::FAILURE as i32
        })
    }
}
