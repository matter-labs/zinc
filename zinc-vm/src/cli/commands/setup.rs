use crate::{Error, IoToError};
use pairing::bn256::Bn256;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use zinc_bytecode::program::Program;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "setup",
    about = "Generates a pair of proving and verifying keys"
)]
pub struct SetupCommand {
    #[structopt(
        short = "c",
        long = "circuit",
        help = "Circuit's bytecode file"
    )]
    pub circuit_path: PathBuf,

    #[structopt(
        short = "p",
        long = "proving-key",
        help = "Params file to write"
    )]
    pub proving_key_path: PathBuf,

    #[structopt(
        short = "v",
        long = "verifying-key",
        help = "Params file to write"
    )]
    pub verifying_key_path: PathBuf,
}

impl SetupCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let bytes =
            fs::read(&self.circuit_path).error_with_path(|| self.circuit_path.to_string_lossy())?;
        let program = Program::from_bytes(bytes.as_slice()).map_err(Error::ProgramDecoding)?;

        let params = zinc_vm::setup::<Bn256>(&program)?;

        let pkey_file = fs::File::create(&self.proving_key_path)
            .error_with_path(|| self.proving_key_path.to_string_lossy())?;
        params
            .write(pkey_file)
            .error_with_path(|| self.proving_key_path.to_string_lossy())?;

        let vk_hex = {
            let mut vk_bytes = Vec::new();
            params.vk.write(&mut vk_bytes).expect("writing to vec");
            hex::encode(vk_bytes) + "\n"
        };

        fs::write(&self.verifying_key_path, vk_hex)
            .error_with_path(|| self.verifying_key_path.to_string_lossy())?;

        Ok(())
    }
}
