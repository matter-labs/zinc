//!
//! The Zinc virtual machine `run` subcommand.
//!

use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_build::Program as BuildProgram;
use zinc_build::Value as BuildValue;

use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `run` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "run", about = "Executes circuit and prints program's output")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary", help = "The bytecode file")]
    pub binary_path: PathBuf,

    /// The path to the witness JSON file.
    #[structopt(long = "witness", help = "The witness JSON file")]
    pub witness_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(long = "public-data", help = "The public data JSON file")]
    pub public_data_path: PathBuf,

    /// The method name to call, if the program is a contract.
    #[structopt(long = "method", help = "The method name")]
    pub method: Option<String>,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        let bytecode =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let input_template = fs::read_to_string(&self.witness_path)
            .error_with_path(|| self.witness_path.to_string_lossy())?;

        let program =
            BuildProgram::from_bytes(bytecode.as_slice()).map_err(Error::ProgramDecoding)?;
        let input_json = serde_json::from_str(&input_template)?;

        let output = match program {
            BuildProgram::Circuit(circuit) => {
                let input_type = circuit.input.clone();
                let input_values = BuildValue::try_from_typed_json(input_json, input_type)?;
                CircuitFacade::new(circuit).run::<Bn256>(input_values)?
            }
            BuildProgram::Contract(contract) => {
                let method_name = self.method.ok_or(Error::MethodNameNotFound)?;
                let method = contract.methods.get(method_name.as_str()).cloned().ok_or(
                    Error::MethodNotFound {
                        name: method_name.clone(),
                    },
                )?;
                let input_values = BuildValue::try_from_typed_json(input_json, method.input)?;
                let (output, storage) = ContractFacade::new(contract).run::<Bn256>(
                    input_values,
                    BuildValue::Contract(vec![]),
                    method_name,
                )?;
                output
            }
        };

        let output_json = serde_json::to_string_pretty(&output.into_json())? + "\n";
        let public_data_path = self.public_data_path;
        fs::write(&public_data_path, &output_json)
            .error_with_path(|| public_data_path.to_string_lossy())?;

        print!("{}", output_json);

        Ok(zinc_const::exit_code::SUCCESS as i32)
    }
}
