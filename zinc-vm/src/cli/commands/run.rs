use crate::Error;
use pairing::bn256::Bn256;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;

#[derive(Debug, StructOpt)]
#[structopt(name = "run", about = "Executes circuit and prints program's output")]
pub struct RunCommand {
    #[structopt(short = "c", long = "circuit", about = "Circuit's bytecode file")]
    pub circuit_path: PathBuf,

    #[structopt(short = "i", long = "input", about = "Program's input file")]
    pub input_path: PathBuf,

    #[structopt(short = "o", long = "output", about = "Program's output file")]
    pub output_path: PathBuf,
}

impl RunCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let bytes = fs::read(&self.circuit_path)?;
        let program = Program::from_bytes(bytes.as_slice()).unwrap();

        let input_text = fs::read_to_string(&self.input_path)?;
        let json = serde_json::from_str(&input_text)?;
        let input: Value = Value::from_typed_json(&json, &program.input)?;

        let output = zinc_vm::run::<Bn256>(&program, &input)?;

        let output_json = serde_json::to_string_pretty(&output.to_json())? + "\n";
        fs::write(&self.output_path, &output_json)?;

        print!("{}", output_json);

        Ok(())
    }
}
