use crate::{Error, IoToError};
use pairing::bn256::Bn256;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;

#[derive(Debug, StructOpt)]
#[structopt(name = "debug", about = "Executes circuit with additional checks")]
pub struct DebugCommand {
    #[structopt(short = "c", long = "circuit", help = "Circuit's bytecode file")]
    pub circuit_path: PathBuf,

    #[structopt(short = "i", long = "input", help = "Program's input file")]
    pub input_path: PathBuf,

    #[structopt(short = "o", long = "output", help = "Program's output file")]
    pub output_path: PathBuf,
}

impl DebugCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let bytes =
            fs::read(&self.circuit_path).error_with_path(|| self.circuit_path.to_string_lossy())?;
        let program = Program::from_bytes(bytes.as_slice()).map_err(Error::ProgramDecoding)?;

        let input_text = fs::read_to_string(&self.input_path)
            .error_with_path(|| self.input_path.to_string_lossy())?;
        let json = serde_json::from_str(&input_text)?;
        let input = Value::from_typed_json(&json, &program.input)?;

        let output = zinc_vm::debug::<Bn256>(&program, &input)?;

        let output_json = serde_json::to_string_pretty(&output.to_json())? + "\n";
        fs::write(&self.output_path, &output_json)
            .error_with_path(|| self.output_path.to_string_lossy())?;

        print!("{}", output_json);

        Ok(())
    }
}
