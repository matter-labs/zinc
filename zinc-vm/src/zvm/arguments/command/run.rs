//!
//! The Zinc virtual machine `run` subcommand.
//!

use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

use serde_json::Value as JsonValue;
use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_build::Application as BuildApplication;
use zinc_build::ContractFieldValue as BuildContractFieldValue;
use zinc_build::InputBuild;
use zinc_build::Value as BuildValue;
use zinc_zksync::TransactionMsg;

use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;
use zinc_vm::ContractInput;

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

    /// The path to the input JSON file.
    #[structopt(long = "input")]
    pub input_path: PathBuf,

    /// The path to the output JSON file.
    #[structopt(long = "output")]
    pub output_path: PathBuf,

    /// The method name to call, if the application is a contract.
    #[structopt(long = "method")]
    pub method: Option<String>,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        // Read the bytecode
        let bytecode =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let application = BuildApplication::try_from_slice(bytecode.as_slice())
            .map_err(Error::ApplicationDecoding)?;

        // Read the input file
        let input_path = self.input_path;
        let input_template =
            fs::read_to_string(&input_path).error_with_path(|| input_path.to_string_lossy())?;
        let input: InputBuild = serde_json::from_str(input_template.as_str())?;

        let output = match application {
            BuildApplication::Circuit(circuit) => match input {
                InputBuild::Circuit { arguments } => {
                    let input_type = circuit.input.clone();
                    let arguments = BuildValue::try_from_typed_json(arguments, input_type)?;

                    CircuitFacade::new(circuit).run::<Bn256>(arguments)?.result
                }
                InputBuild::Contract { .. } => {
                    return Err(Error::InputDataInvalid {
                        expected: "circuit".to_owned(),
                        found: "contract".to_owned(),
                    })
                }
            },
            BuildApplication::Contract(contract) => match input {
                InputBuild::Circuit { .. } => {
                    return Err(Error::InputDataInvalid {
                        expected: "contract".to_owned(),
                        found: "circuit".to_owned(),
                    })
                }
                InputBuild::Contract {
                    arguments,
                    msg: transaction,
                    storage,
                } => {
                    let method_name = self.method.ok_or(Error::MethodNameNotFound)?;
                    let method = contract.methods.get(method_name.as_str()).cloned().ok_or(
                        Error::MethodNotFound {
                            name: method_name.clone(),
                        },
                    )?;

                    let method_arguments = arguments.get(method_name.as_str()).cloned().ok_or(
                        Error::MethodArgumentsNotFound {
                            name: method_name.clone(),
                        },
                    )?;
                    let method_arguments =
                        BuildValue::try_from_typed_json(method_arguments, method.input)?;

                    let storage_size = contract.storage.len();
                    let storage_values = match storage {
                        JsonValue::Array(array) => {
                            let mut storage_values = Vec::with_capacity(contract.storage.len());
                            for (field, value) in contract.storage.clone().into_iter().zip(array) {
                                storage_values.push(BuildContractFieldValue::new(
                                    field.name,
                                    BuildValue::try_from_typed_json(value, field.r#type)?,
                                    field.is_public,
                                    field.is_implicit,
                                ));
                            }
                            storage_values
                        }
                        value => return Err(Error::InvalidContractStorageFormat { found: value }),
                    };

                    let output = ContractFacade::new(contract).run::<Bn256>(ContractInput::new(
                        method_arguments,
                        BuildValue::Contract(storage_values),
                        method_name,
                        TransactionMsg::try_from(&transaction).map_err(|error| {
                            Error::InvalidTransaction {
                                inner: error,
                                found: transaction.clone(),
                            }
                        })?,
                    ))?;

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

                    let input_str = serde_json::to_string_pretty(&InputBuild::new_contract(
                        JsonValue::Array(storage_values),
                        transaction,
                        arguments,
                    ))
                    .expect(zinc_const::panic::DATA_CONVERSION);
                    fs::write(&input_path, input_str)
                        .error_with_path(|| input_path.to_string_lossy())?;

                    output.result
                }
            },
        };

        let output_json = serde_json::to_string_pretty(&output.into_json())? + "\n";
        let output_path = self.output_path;
        fs::write(&output_path, &output_json).error_with_path(|| output_path.to_string_lossy())?;

        print!("{}", output_json);

        Ok(zinc_const::exit_code::SUCCESS as i32)
    }
}
