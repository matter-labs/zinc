//!
//! The Zinc virtual machine `verify` subcommand.
//!

use std::fs;
use std::path::PathBuf;

use colored::Colorize;
use structopt::StructOpt;

use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::groth16::VerifyingKey;
use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;

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
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        // Read proof
        let proof_bytes = read_hex(std::io::stdin(), "<stdin>", "proof")?;
        let proof =
            Proof::<Bn256>::read(proof_bytes.as_slice()).error_with_path(|| "<proof data>")?;

        // Read program
        let bytes =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let program =
            BytecodeProgram::from_bytes(bytes.as_slice()).map_err(Error::ProgramDecoding)?;

        // Read verification key
        let key_file = fs::File::open(&self.verifying_key_path)
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;
        let key_bytes = read_hex(
            key_file,
            &self.verifying_key_path.to_string_lossy(),
            "verification key",
        )?;
        let key = VerifyingKey::<Bn256>::read(key_bytes.as_slice())
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;

        // Read public input
        let output_text = fs::read_to_string(&self.public_data_path)
            .error_with_path(|| self.public_data_path.to_string_lossy())?;
        let output_value = serde_json::from_str(output_text.as_str())?;
        let output_struct = TemplateValue::try_from_typed_json(output_value, program.output())?;

        // Verify
        let verified = Facade::verify::<Bn256>(key, proof, output_struct)?;

        Ok(if verified {
            println!("{}", "✔  Verified".bold().green());
            zinc_const::exit_code::SUCCESS as i32
        } else {
            println!("{}", "✘  Failed".bold().red());
            zinc_const::exit_code::FAILURE as i32
        })
    }
}

///
/// Reads hex data from the `reader`. Used mainly for reading proofs and keys.
///
fn read_hex<R: std::io::Read>(
    mut reader: R,
    path_hint: &str,
    context_hint: &str,
) -> Result<Vec<u8>, Error> {
    let mut hex = String::new();
    reader
        .read_to_string(&mut hex)
        .error_with_path(|| path_hint)?;

    let bytes = hex::decode(hex.trim()).map_err(|error| Error::HexDecoding {
        context: context_hint.into(),
        error,
    })?;

    Ok(bytes)
}
