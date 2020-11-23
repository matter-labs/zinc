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

use zinc_vm::Facade;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `verify` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "verify", about = "Verifies a proof using the verifying key")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary")]
    pub binary_path: PathBuf,

    /// The path to the verifying key file.
    #[structopt(long = "verifying-key")]
    pub verifying_key_path: PathBuf,

    /// The path to the output JSON file.
    #[structopt(long = "output")]
    pub output_path: PathBuf,

    /// The method name to call, if the application is a contract.
    #[structopt(long = "method")]
    pub method: Option<String>,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        // Read the proof
        let mut proof = String::new();
        std::io::stdin()
            .read_to_string(&mut proof)
            .error_with_path(|| "<stdin>")?;
        let proof = hex::decode(proof.trim()).map_err(|error| Error::HexDecoding {
            context: "proof".to_owned(),
            error,
        })?;
        let proof = Proof::<Bn256>::read(proof.as_slice()).error_with_path(|| "<proof data>")?;

        // Read the application
        let bytes =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let application = zinc_build::Application::try_from_slice(bytes.as_slice())
            .map_err(Error::ApplicationDecoding)?;

        // Read the verification key
        let mut verifying_key_file = fs::File::open(&self.verifying_key_path)
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;
        let mut verifying_key = Vec::new();
        verifying_key_file
            .read_to_end(&mut verifying_key)
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;
        let verifying_key = VerifyingKey::<Bn256>::read(verifying_key.as_slice())
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;

        // Read the public input
        let output_text = fs::read_to_string(&self.output_path)
            .error_with_path(|| self.output_path.to_string_lossy())?;
        let output_json = serde_json::from_str(output_text.as_str())?;
        let output_type = match application {
            zinc_build::Application::Circuit(circuit) => circuit.output,
            zinc_build::Application::Contract(contract) => {
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
        let output_value = zinc_build::Value::try_from_typed_json(output_json, output_type)?;

        // Verify the proof
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
