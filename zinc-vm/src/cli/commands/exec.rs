use crate::data_io::json_to_flat_input;
use crate::Error;
use pairing::bn256::Bn256;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use zinc_bytecode::program::Program;

#[derive(Debug, StructOpt)]
#[structopt(name = "exec", about = "Executes circuit and prints program's output")]
pub struct ExecCommand {
    #[structopt(short = "c", long = "circuit", about = "Circuit's bytecode file")]
    pub circuit_file: PathBuf,

    #[structopt(short = "i", long = "input", about = "Program's input file")]
    pub input_file: PathBuf,

    #[structopt(short = "o", long = "output", about = "Program's output file")]
    pub output_file: PathBuf,
}

impl ExecCommand {
    pub fn execute(&self) -> Result<(), Error> {
        let bytes = fs::read(&self.circuit_file)?;
        let program = Program::from_bytes(bytes.as_slice()).unwrap();

        let input_text = fs::read_to_string(&self.input_file)?;
        let input_json = json::parse(input_text.as_str())?;
        // TODO: Remove unwrap
        let input = json_to_flat_input(&input_json).unwrap();

        let outputs = zinc_vm::exec::<Bn256>(&program, input.as_slice())?;

        for value in outputs.iter() {
            match value {
                None => println!("None"),
                Some(value) => println!("{}", value),
            }
        }

        Ok(())
    }
}
