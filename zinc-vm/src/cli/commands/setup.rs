use crate::Error;
use pairing::bn256::Bn256;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use zinc_bytecode::program::Program;

#[derive(Debug, StructOpt)]
pub struct SetupCommand {
    #[structopt(short = "c", long = "circuit", about = "Circuit's bytecode file")]
    pub circuit_path: PathBuf,

    #[structopt(short = "p", long = "proving-key", about = "Params file to write")]
    pub proving_key_path: PathBuf,

    #[structopt(short = "v", long = "verifying-key", about = "Params file to write")]
    pub verifying_key_path: PathBuf,
}

impl SetupCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let bytes = fs::read(&self.circuit_path)?;
        let program = Program::from_bytes(bytes.as_slice()).unwrap();

        let params = zinc_vm::setup::<Bn256>(&program)?;

        let pkey_file = fs::File::create(&self.proving_key_path)?;
        params.write(pkey_file)?;

        let vk_hex = {
            let mut vk_bytes = Vec::new();
            params.vk.write(&mut vk_bytes)?;
            hex::encode(vk_bytes)
        };

        let mut vkey_file = fs::File::create(&self.verifying_key_path)?;
        writeln!(vkey_file, "{}", vk_hex)?;

        Ok(())
    }
}
