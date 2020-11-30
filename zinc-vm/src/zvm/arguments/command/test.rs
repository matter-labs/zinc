//!
//! The Zinc virtual machine `test` subcommand.
//!

use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `test` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "test", about = "Executes a unit test")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary")]
    pub binary_path: PathBuf,

    /// The path to the input JSON file.
    #[structopt(long = "input")]
    pub input_path: PathBuf,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<i32, Self::Error> {
        // Read the bytecode
        let bytecode =
            fs::read(&self.binary_path).error_with_path(|| self.binary_path.to_string_lossy())?;
        let application = zinc_build::Application::try_from_slice(bytecode.as_slice())
            .map_err(Error::ApplicationDecoding)?;

        // Read the input file
        let input_path = self.input_path;
        let input_template =
            fs::read_to_string(&input_path).error_with_path(|| input_path.to_string_lossy())?;
        let input: zinc_build::InputBuild = serde_json::from_str(input_template.as_str())?;

        let status = match application {
            zinc_build::Application::Circuit(circuit) => {
                CircuitFacade::new(circuit).test::<Bn256>()?
            }
            zinc_build::Application::Contract(contract) => match input {
                zinc_build::InputBuild::Contract {
                    storage,
                    msg: transaction,
                    ..
                } => {
                    let storage_values = match storage {
                        serde_json::Value::Array(array) => {
                            let mut storage_values = Vec::with_capacity(contract.storage.len());
                            for (field, value) in contract.storage.clone().into_iter().zip(array) {
                                storage_values.push(zinc_build::ContractFieldValue::new(
                                    field.name,
                                    zinc_build::Value::try_from_typed_json(value, field.r#type)?,
                                    field.is_public,
                                    field.is_implicit,
                                ));
                            }
                            storage_values
                        }
                        value => return Err(Error::InvalidContractStorageFormat { found: value }),
                    };

                    let transaction =
                        zinc_zksync::TransactionMsg::try_from(&transaction).map_err(|error| {
                            Error::InvalidTransaction {
                                inner: error,
                                found: transaction,
                            }
                        })?;

                    ContractFacade::new(contract)
                        .test::<Bn256>(zinc_build::Value::Contract(storage_values), transaction)?
                }
                zinc_build::InputBuild::Circuit { .. } => {
                    panic!(zinc_const::panic::VALIDATED_DURING_TARGET_CODE_GENERATION)
                }
            },
        };

        Ok(status as i32)
    }
}
