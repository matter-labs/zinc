use structopt::StructOpt;
use std::path::PathBuf;
use crate::Error;
use zinc_bytecode::decode_all_instructions;
use std::fs;
use pairing::bn256::Bn256;

#[derive(Debug, StructOpt)]
pub struct SetupCommand {
    #[structopt(short = "c", long = "circuit", about = "Circuit's bytecode file")]
    pub circuit_path: PathBuf,

    #[structopt(short = "P", long = "params", about = "Params file to write")]
    pub params_path: PathBuf
}

impl SetupCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let bytes = fs::read(&self.circuit_path)?;
        let code = decode_all_instructions(bytes.as_slice())?;

        let params = zinc_vm::setup::<Bn256>(code.as_slice())?;

        let file = fs::File::create(&self.params_path)?;
        params.write(file)?;

        Ok(())
    }
}
