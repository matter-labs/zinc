//!
//! The Zinc virtual machine `run` subcommand.
//!

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

use num::BigInt;
use num::Zero;
use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

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
        let application = zinc_types::Application::try_from_slice(bytecode.as_slice())
            .map_err(Error::ApplicationDecoding)?;

        // Read the input file
        let input_path = self.input_path;
        let input_template =
            fs::read_to_string(&input_path).error_with_path(|| input_path.to_string_lossy())?;
        let input: zinc_types::InputBuild = serde_json::from_str(input_template.as_str())?;

        let output = match application {
            zinc_types::Application::Circuit(circuit) => match input {
                zinc_types::InputBuild::Circuit { arguments } => {
                    let input_type = circuit.input.clone();
                    let arguments = zinc_types::Value::try_from_typed_json(arguments, input_type)?;

                    CircuitFacade::new(circuit).run::<Bn256>(arguments)?.result
                }
                zinc_types::InputBuild::Contract { .. } => {
                    return Err(Error::InputDataInvalid {
                        expected: "circuit".to_owned(),
                        found: "contract".to_owned(),
                    })
                }
                zinc_types::InputBuild::Library { .. } => {
                    return Err(Error::InputDataInvalid {
                        expected: "circuit".to_owned(),
                        found: "library".to_owned(),
                    })
                }
            },
            zinc_types::Application::Contract(contract) => match input {
                zinc_types::InputBuild::Circuit { .. } => {
                    return Err(Error::InputDataInvalid {
                        expected: "contract".to_owned(),
                        found: "circuit".to_owned(),
                    })
                }
                zinc_types::InputBuild::Contract {
                    arguments,
                    msg: transaction,
                    storages,
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
                    let mut method_arguments =
                        zinc_types::Value::try_from_typed_json(method_arguments, method.input)?;
                    if method_name != zinc_const::contract::CONSTRUCTOR_IDENTIFIER {
                        method_arguments.insert_contract_instance(BigInt::zero());
                    }

                    let mut input_storages = HashMap::with_capacity(storages.len());
                    for (address, value) in storages.into_iter() {
                        let address: zksync_types::Address = address["0x".len()..]
                            .parse()
                            .expect(zinc_const::panic::DATA_CONVERSION);

                        let value = match value {
                            serde_json::Value::Array(array) => {
                                let mut storage_values = Vec::with_capacity(contract.storage.len());
                                for (field, value) in
                                    contract.storage.clone().into_iter().zip(array)
                                {
                                    storage_values.push(zinc_types::ContractFieldValue::new(
                                        field.name,
                                        zinc_types::Value::try_from_typed_json(
                                            value,
                                            field.r#type,
                                        )?,
                                        field.is_public,
                                        field.is_implicit,
                                    ));
                                }
                                zinc_types::Value::Contract(storage_values)
                            }
                            value => {
                                return Err(Error::InvalidContractStorageFormat { found: value })
                            }
                        };
                        input_storages.insert(address, value);
                    }

                    let output = ContractFacade::new(contract).run::<Bn256>(ContractInput::new(
                        method_arguments,
                        input_storages,
                        method_name,
                        zinc_types::TransactionMsg::try_from(&transaction).map_err(|error| {
                            Error::InvalidTransaction {
                                inner: error,
                                found: transaction.clone(),
                            }
                        })?,
                    ))?;

                    let mut storages = HashMap::with_capacity(output.storages.len());
                    for (eth_address, value) in output.storages.into_iter() {
                        match value {
                            zinc_types::Value::Contract(fields) => {
                                let mut storage_values = Vec::with_capacity(fields.len());
                                for field in fields.into_iter() {
                                    storage_values.push(field.value.into_json());
                                }
                                storages.insert(
                                    format!(
                                        "0x{}",
                                        eth_address.to_str_radix(zinc_const::base::HEXADECIMAL)
                                    ),
                                    serde_json::Value::Array(storage_values),
                                );
                            }
                            value => {
                                return Err(Error::InvalidContractStorageFormat {
                                    found: value.into_json(),
                                })
                            }
                        }
                    }

                    let input_str = serde_json::to_string_pretty(
                        &zinc_types::InputBuild::new_contract(storages, transaction, arguments),
                    )
                    .expect(zinc_const::panic::DATA_CONVERSION);
                    fs::write(&input_path, input_str)
                        .error_with_path(|| input_path.to_string_lossy())?;

                    output.result
                }
                zinc_types::InputBuild::Library { .. } => {
                    return Err(Error::InputDataInvalid {
                        expected: "contract".to_owned(),
                        found: "library".to_owned(),
                    })
                }
            },
            zinc_types::Application::Library(_library) => return Err(Error::CannotRunLibrary),
        };

        let output_json = serde_json::to_string_pretty(&output.into_json())? + "\n";
        let output_path = self.output_path;
        fs::write(&output_path, &output_json).error_with_path(|| output_path.to_string_lossy())?;

        print!("{}", output_json);

        Ok(zinc_const::exit_code::SUCCESS as i32)
    }
}
