//!
//! The Zinc virtual machine `prove` subcommand.
//!

use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;

use structopt::StructOpt;

use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::pairing::bn256::Bn256;

use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;
use zinc_vm::ContractInput;

use crate::arguments::command::IExecutable;
use crate::error::Error;
use crate::error::IErrorPath;

///
/// The Zinc virtual machine `prove` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(name = "prove", about = "Generates a proof using the proving key")]
pub struct Command {
    /// The path to the binary bytecode file.
    #[structopt(long = "binary")]
    pub binary_path: PathBuf,

    /// The path to the proving key file.
    #[structopt(long = "proving-key")]
    pub proving_key_path: PathBuf,

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
        let application = zinc_build::Application::try_from_slice(bytecode.as_slice())
            .map_err(Error::ApplicationDecoding)?;

        // Read the input file
        let input_template = fs::read_to_string(&self.input_path)
            .error_with_path(|| self.input_path.to_string_lossy())?;
        let input: zinc_build::InputBuild = serde_json::from_str(input_template.as_str())?;

        // Read the proving key
        let proving_key_path = self.proving_key_path;
        let file = fs::File::open(&proving_key_path)
            .error_with_path(|| proving_key_path.to_string_lossy())?;
        let params = Parameters::<Bn256>::read(file, true)
            .error_with_path(|| proving_key_path.to_string_lossy())?;

        let proof = match application {
            zinc_build::Application::Circuit(circuit) => match input {
                zinc_build::InputBuild::Circuit { arguments } => {
                    let input_type = circuit.input.clone();
                    let arguments = zinc_build::Value::try_from_typed_json(arguments, input_type)?;

                    let (_output, proof) =
                        CircuitFacade::new(circuit).prove::<Bn256>(params, arguments)?;

                    proof
                }
                zinc_build::InputBuild::Contract { .. } => {
                    return Err(Error::InputDataInvalid {
                        expected: "circuit".to_owned(),
                        found: "contract".to_owned(),
                    })
                }
            },
            zinc_build::Application::Contract(contract) => match input {
                zinc_build::InputBuild::Circuit { .. } => {
                    return Err(Error::InputDataInvalid {
                        expected: "contract".to_owned(),
                        found: "circuit".to_owned(),
                    })
                }
                zinc_build::InputBuild::Contract {
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
                        zinc_build::Value::try_from_typed_json(method_arguments, method.input)?;

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

                    let (_output, proof) = ContractFacade::new(contract).prove::<Bn256>(
                        params,
                        ContractInput::new(
                            method_arguments,
                            zinc_build::Value::Contract(storage_values),
                            method_name,
                            zinc_zksync::TransactionMsg::try_from(&transaction).map_err(
                                |error| Error::InvalidTransaction {
                                    inner: error,
                                    found: transaction,
                                },
                            )?,
                        ),
                    )?;

                    proof
                }
            },
        };

        // Write the proof to stdout
        let mut proof_bytes = Vec::new();
        proof.write(&mut proof_bytes).expect("writing to vec");
        let proof_hex = hex::encode(proof_bytes);
        println!("{}", proof_hex);

        Ok(zinc_const::exit_code::SUCCESS as i32)
    }
}
