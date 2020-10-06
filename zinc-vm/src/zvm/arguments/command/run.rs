//!
//! The Zinc virtual machine `run` subcommand.
//!

use std::fs;
use std::path::PathBuf;

use serde_json::Value as JsonValue;
use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_build::Application as BuildApplication;
use zinc_build::ContractFieldValue as BuildContractFieldValue;
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
#[structopt(name = "run", about = "Executes the bytecode and prints its output")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary")]
    pub binary_path: PathBuf,

    /// The path to the witness JSON file.
    #[structopt(long = "witness")]
    pub witness_path: PathBuf,

    /// The path to the public data JSON file.
    #[structopt(long = "public-data")]
    pub public_data_path: PathBuf,

    /// The path to the contract storage JSON file.
    #[structopt(long = "storage")]
    pub storage: Option<PathBuf>,

    /// The method name to call, if the application is a contract.
    #[structopt(long = "method")]
    pub method: Option<String>,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(mut self) -> Result<i32, Self::Error> {
        let bytecode =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let input_template = fs::read_to_string(&self.witness_path)
            .error_with_path(|| self.witness_path.to_string_lossy())?;

        let application = BuildApplication::try_from_slice(bytecode.as_slice())
            .map_err(Error::ApplicationDecoding)?;
        let input_json = serde_json::from_str(input_template.as_str())?;

        let output = match application {
            BuildApplication::Circuit(circuit) => {
                let input_type = circuit.input.clone();
                let input_values = BuildValue::try_from_typed_json(input_json, input_type)?;
                CircuitFacade::new(circuit)
                    .run::<Bn256>(input_values)?
                    .result
            }
            BuildApplication::Contract(contract) => {
                let storage_path = match self.storage.take() {
                    Some(path) => path,
                    None => return Err(Error::ContractStoragePathMissing),
                };

                let storage_str = fs::read_to_string(&storage_path)
                    .error_with_path(|| self.witness_path.to_string_lossy())?;
                let storage_json = serde_json::from_str(storage_str.as_str())?;
                let storage_size = contract.storage.len();
                let storage_values = match storage_json {
                    JsonValue::Array(array) => {
                        let mut storage_values = Vec::with_capacity(contract.storage.len());
                        for (field, value) in contract.storage.clone().into_iter().zip(array) {
                            storage_values.push(BuildContractFieldValue::new(
                                field.name,
                                BuildValue::try_from_typed_json(value, field.r#type)?,
                                field.is_public,
                                field.is_external,
                            ));
                        }
                        storage_values
                    }
                    value => return Err(Error::InvalidContractStorageFormat { found: value }),
                };

                let method_name = self.method.ok_or(Error::MethodNameNotFound)?;
                let method = contract.methods.get(method_name.as_str()).cloned().ok_or(
                    Error::MethodNotFound {
                        name: method_name.clone(),
                    },
                )?;
                let input_values = BuildValue::try_from_typed_json(input_json, method.input)?;
                let output = ContractFacade::new(contract).run::<Bn256>(
                    input_values,
                    BuildValue::Contract(storage_values),
                    method_name,
                )?;

                let mut storage_values = Vec::with_capacity(storage_size);
                match output.storage {
                    BuildValue::Contract(fields) => {
                        for field in fields.into_iter() {
                            storage_values.push(field.value.into_json());
                        }
                    }
                    value => {
                        return Err(Error::InvalidContractStorageFormat {
                            found: value.into_json(),
                        })
                    }
                }
                let storage_str = serde_json::to_string_pretty(&JsonValue::Array(storage_values))
                    .expect(zinc_const::panic::DATA_CONVERSION);
                fs::write(&storage_path, storage_str)
                    .error_with_path(|| storage_path.to_string_lossy())?;

                output.result
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
