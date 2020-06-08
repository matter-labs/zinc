use std::fs;
use std::path::PathBuf;
use std::process;

use colored::Colorize;
use structopt::StructOpt;

use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::groth16::VerifyingKey;
use pairing::bn256::Bn256;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;

use zinc_vm::IFacade;

use crate::error::Error;
use crate::error::IoToError;

#[derive(Debug, StructOpt)]
#[structopt(name = "verify", about = "Verifies the proof using verifying key")]
pub struct VerifyCommand {
    #[structopt(long = "binary", help = "The bytecode file")]
    pub binary_path: PathBuf,

    #[structopt(long = "verifying-key", help = "The verifying key path")]
    pub veryfying_key_path: PathBuf,

    #[structopt(long = "public-data", help = "Path to public data JSON file")]
    pub public_data_path: PathBuf,
}

impl VerifyCommand {
    pub fn execute(&self) -> Result<(), Error> {
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
        let key_file = fs::File::open(&self.veryfying_key_path)
            .error_with_path(|| self.veryfying_key_path.to_string_lossy())?;
        let key_bytes = read_hex(
            key_file,
            &self.veryfying_key_path.to_string_lossy(),
            "verification key",
        )?;
        let key = VerifyingKey::<Bn256>::read(key_bytes.as_slice())
            .error_with_path(|| self.veryfying_key_path.to_string_lossy())?;

        // Read public input
        let output_text = fs::read_to_string(&self.public_data_path)
            .error_with_path(|| self.public_data_path.to_string_lossy())?;
        let output_value = serde_json::from_str(output_text.as_str())?;
        let output_struct = TemplateValue::from_typed_json(&output_value, &program.output())?;

        // Verify
        let verified = BytecodeProgram::verify::<Bn256>(key, proof, output_struct)?;

        if verified {
            println!("{}", "✔  Verified".bold().green());
        } else {
            println!("{}", "✘  Failed".bold().red());
            process::exit(1);
        }

        Ok(())
    }
}

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
