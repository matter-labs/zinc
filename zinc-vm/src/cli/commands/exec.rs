use crate::Error;
use pairing::bn256::Bn256;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use zinc_bytecode::data::values::Value;
use zinc_bytecode::program::Program;

#[derive(Debug, StructOpt)]
#[structopt(name = "exec", about = "Executes circuit and prints program's output")]
pub struct ExecCommand {
    #[structopt(short = "c", long = "circuit", about = "Circuit's bytecode file")]
    pub circuit_path: PathBuf,

    #[structopt(short = "i", long = "input", about = "Program's input file")]
    pub input_path: PathBuf,

    #[structopt(short = "o", long = "output", about = "Program's output file")]
    pub output_path: PathBuf,
}

impl ExecCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let bytes = fs::read(&self.circuit_path)?;
        let program = Program::from_bytes(bytes.as_slice()).unwrap();

        let input_text = fs::read_to_string(&self.input_path)?;
        let input_values: Value = serde_json::from_str(&input_text)?;
        let input = input_values.to_flat_values();

        let output_values = zinc_vm::exec::<Bn256>(&program, &input)?;

        // TODO: Remove unwrap
        let output = Value::from_flat_values(&program.output, &output_values).unwrap();

        let output_json = serde_json::to_string_pretty(&output)? + "\n";
        fs::write(&self.output_path, &output_json)?;

        print!("{}", &output_json);

        Ok(())
    }
}
