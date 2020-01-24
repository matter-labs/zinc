use crate::Error;
use colored::Colorize;
use franklin_crypto::bellman::groth16::{Parameters, Proof};
use pairing::bn256::Bn256;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;
use zinc_bytecode::data::values::Value;
use std::io::Read;

#[derive(Debug, StructOpt)]
pub struct VerifyCommand {
    #[structopt(short = "c", long = "circuit", about = "Circuit's bytecode file")]
    pub circuit_file: PathBuf,

    #[structopt(short = "P", long = "params", about = "Circuit's bytecode file")]
    pub params_path: PathBuf,

    #[structopt(short = "p", long = "proof", about = "Proof file")]
    pub proof_path: PathBuf,

    #[structopt(short = "o", long = "output", about = "Program's output file")]
    pub output_path: PathBuf,
}

impl VerifyCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let params_file = fs::File::open(&self.params_path)?;
        let params = Parameters::<Bn256>::read(params_file, true)?;

        let mut stdin = std::io::stdin();
        let mut proof_hex_bytes = Vec::new();
        stdin.read_to_end(&mut proof_hex_bytes).expect("failed to read from stdin");
        let proof_hex = String::from_utf8(proof_hex_bytes).expect("invalid utf-8");
        let proof_bytes = hex::decode(proof_hex.trim()).expect("failed to decode hex proof");
        let proof = Proof::<Bn256>::read(proof_bytes.as_slice())?;

        let output_text = fs::read_to_string(&self.output_path)?;
        let output_value: Value = serde_json::from_str(output_text.as_str())?;
        let output = output_value.to_flat_values();

        let verified = zinc_vm::verify(&params, &proof, &output)?;

        if verified {
            println!("{}", "✔  Verified".bold().green());
        } else {
            println!("{}", "❌  Failed".bold().red());
            exit(1);
        }

        Ok(())
    }
}
