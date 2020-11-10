//!
//! The virtual machine contract facade.
//!

use std::marker::PhantomData;

use colored::Colorize;
use num::BigInt;

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_build::Contract as BuildContract;
use zinc_build::ContractFieldValue;
use zinc_const::UnitTestExitCode;
use zinc_zksync::TransactionMsg;

use crate::constraint_systems::constant::Constant as ConstantCS;
use crate::core::contract::input::Input as ContractInput;
use crate::core::contract::output::Output as ContractOutput;
use crate::core::contract::storage::database::Storage as DatabaseStorage;
use crate::core::contract::storage::leaf::LeafInput;
use crate::core::contract::storage::leaf::LeafOutput;
use crate::core::contract::storage::setup::Storage as SetupStorage;
use crate::core::contract::synthesizer::Synthesizer as ContractSynthesizer;
use crate::core::contract::State as ContractState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::contract::merkle_tree::hasher::sha256::Hasher as Sha256Hasher;
use crate::gadgets::contract::merkle_tree::IMerkleTree;
use crate::gadgets::contract::storage::StorageGadget;
use crate::IEngine;

pub struct Facade {
    inner: BuildContract,
}

impl Facade {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(inner: BuildContract) -> Self {
        Self { inner }
    }

    pub fn run<E: IEngine>(self, input: ContractInput) -> Result<ContractOutput, Error> {
        let mut cs = ConstantCS {};

        let method = self
            .inner
            .methods
            .get(input.method_name.as_str())
            .cloned()
            .ok_or(Error::MethodNotFound {
                found: input.method_name.clone(),
            })?;

        let arguments_flat = input.arguments.into_flat_values();
        let output_type = if method.is_mutable {
            method.output.into_mutable_method_output()
        } else {
            method.output
        };

        let storage_fields = self.inner.storage.clone();
        let mut storage_types = Vec::with_capacity(self.inner.storage.len());
        for field in self.inner.storage.iter() {
            storage_types.push(field.r#type.to_owned());
        }
        let storage_leaves = match input.storage {
            zinc_build::Value::Contract(fields) => fields
                .into_iter()
                .enumerate()
                .map(|(index, field)| {
                    let r#type = storage_types[index].to_owned();

                    match field.value {
                        zinc_build::Value::Map(map) => {
                            let (key_type, value_type) = match r#type {
                                zinc_build::Type::Map {
                                    key_type,
                                    value_type,
                                } => (*key_type, *value_type),
                                _ => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
                            };

                            let entries = map
                                .into_iter()
                                .map(|(key, value)| {
                                    (key.into_flat_values(), value.into_flat_values())
                                })
                                .collect();
                            LeafInput::Map {
                                key_type,
                                value_type,
                                entries,
                            }
                        }
                        value => {
                            let mut values = value.into_flat_values();
                            values.reverse();
                            LeafInput::Array { r#type, values }
                        }
                    }
                })
                .collect::<Vec<LeafInput>>(),
            _ => return Err(Error::InvalidStorageValue),
        };
        let storage = DatabaseStorage::<Bn256>::new(storage_leaves);
        let storage_gadget =
            StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;

        let mut state =
            ContractState::new(cs, storage_gadget, input.method_name, input.transaction);

        let mut num_constraints = 0;
        let result = state.run(
            self.inner,
            method.input,
            Some(&arguments_flat),
            |cs| {
                let num = cs.num_constraints() - num_constraints;
                num_constraints += num;
                log::trace!("Constraints: {}", num);
            },
            |cs| {
                if !cs.is_satisfied() {
                    return Err(Error::UnsatisfiedConstraint);
                }

                Ok(())
            },
            method.address,
        )?;

        let cs = state.constraint_system();
        if !cs.is_satisfied() {
            return Err(Error::UnsatisfiedConstraint);
        }

        let output_value: Vec<BigInt> = result.into_iter().filter_map(|value| value).collect();
        let output_value = zinc_build::Value::from_flat_values(output_type, &output_value);

        let storage_value = zinc_build::Value::Contract(
            state
                .storage
                .into_inner()
                .into_values()
                .into_iter()
                .zip(storage_fields)
                .enumerate()
                .map(|(index, (leaf, field))| {
                    let r#type = storage_types
                        .get(index)
                        .cloned()
                        .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);

                    let value = match leaf {
                        LeafOutput::Array(array) => {
                            zinc_build::Value::from_flat_values(r#type, array.as_slice())
                        }
                        LeafOutput::Map(entries) => {
                            let (key_type, value_type) = match r#type {
                                zinc_build::Type::Map {
                                    key_type,
                                    value_type,
                                } => (*key_type, *value_type),
                                _ => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
                            };

                            let mut values = Vec::with_capacity(entries.len());
                            for (key, value) in entries.into_iter() {
                                let key = zinc_build::Value::from_flat_values(
                                    key_type.clone(),
                                    key.as_slice(),
                                );
                                let value = zinc_build::Value::from_flat_values(
                                    value_type.clone(),
                                    value.as_slice(),
                                );
                                values.push((key, value));
                            }
                            zinc_build::Value::Map(values)
                        }
                    };

                    ContractFieldValue::new(field.name, value, field.is_public, field.is_implicit)
                })
                .collect::<Vec<ContractFieldValue>>(),
        );

        let transfers = state.execution_state.transfers;

        Ok(ContractOutput::new(output_value, storage_value, transfers))
    }

    pub fn test<E: IEngine>(self) -> Result<UnitTestExitCode, Error> {
        let mut exit_code = UnitTestExitCode::Passed;

        for (name, unit_test) in self.inner.unit_tests.clone().into_iter() {
            if unit_test.is_ignored {
                println!("test {} ... {}", name, "ignore".yellow());
                return Ok(UnitTestExitCode::Ignored);
            }

            let mut cs = TestConstraintSystem::<Bn256>::new();

            let storage_types = self
                .inner
                .storage
                .clone()
                .into_iter()
                .map(|field| field.r#type)
                .collect::<Vec<zinc_build::Type>>();
            let storage = SetupStorage::new(storage_types);
            let storage_gadget =
                StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;

            let mut state =
                ContractState::new(cs, storage_gadget, name.clone(), TransactionMsg::default());

            let result = state.run(
                self.inner.clone(),
                zinc_build::Type::new_empty_structure(),
                Some(&[]),
                |_| {},
                |_| Ok(()),
                unit_test.address,
            );

            match result {
                Err(_) if unit_test.should_panic => {
                    println!("test {} ... {} (failed)", name, "ok".green());
                }
                Ok(_) if unit_test.should_panic => {
                    println!(
                        "test {} ... {} (should have failed)",
                        name,
                        "error".bright_red()
                    );
                    exit_code = UnitTestExitCode::Failed;
                }

                Ok(_) => {
                    println!("test {} ... {}", name, "ok".green());
                }
                Err(error) => {
                    println!("test {} ... {} ({})", name, "error".bright_red(), error);
                    exit_code = UnitTestExitCode::Failed;
                }
            };
        }

        Ok(exit_code)
    }

    pub fn setup<E: IEngine>(self, method_name: String) -> Result<Parameters<E>, Error> {
        let rng = &mut rand::thread_rng();
        let mut result = None;

        let method = self
            .inner
            .methods
            .get(method_name.as_str())
            .cloned()
            .ok_or(Error::MethodNotFound {
                found: method_name.clone(),
            })?;

        let storage_fields = self
            .inner
            .storage
            .iter()
            .map(|field| field.r#type.to_owned())
            .collect();
        let storage = SetupStorage::new(storage_fields);

        let synthesizable = ContractSynthesizer {
            inputs: None,
            output: &mut result,
            bytecode: self.inner,
            method,
            storage,
            transaction: TransactionMsg::default(),

            _pd: PhantomData,
        };

        let params = groth16::generate_random_parameters::<E, _, _>(synthesizable, rng)?;

        match result.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS) {
            Ok(_) => Ok(params),
            Err(error) => Err(error),
        }
    }

    pub fn prove<E: IEngine>(
        self,
        params: Parameters<E>,
        input: ContractInput,
    ) -> Result<(zinc_build::Value, Proof<E>), Error> {
        let method = self
            .inner
            .methods
            .get(input.method_name.as_str())
            .cloned()
            .ok_or(Error::MethodNotFound {
                found: input.method_name.clone(),
            })?;

        let mut result = None;
        let rng = &mut rand::thread_rng();

        let arguments_flat = input.arguments.into_flat_values();
        let output_type = if method.is_mutable {
            method.output.clone().into_mutable_method_output()
        } else {
            method.output.clone()
        };

        let mut storage_types = Vec::with_capacity(self.inner.storage.len());
        for field in self.inner.storage.iter() {
            storage_types.push(field.r#type.to_owned());
        }
        let storage_leaves = match input.storage {
            zinc_build::Value::Contract(fields) => fields
                .into_iter()
                .enumerate()
                .map(|(index, field)| {
                    let r#type = storage_types[index].to_owned();

                    match field.value {
                        zinc_build::Value::Map(map) => {
                            let (key_type, value_type) = match r#type {
                                zinc_build::Type::Map {
                                    key_type,
                                    value_type,
                                } => (*key_type, *value_type),
                                _ => panic!(zinc_const::panic::VALIDATED_DURING_SEMANTIC_ANALYSIS),
                            };

                            let entries = map
                                .into_iter()
                                .map(|(key, value)| {
                                    (key.into_flat_values(), value.into_flat_values())
                                })
                                .collect();
                            LeafInput::Map {
                                key_type,
                                value_type,
                                entries,
                            }
                        }
                        value => {
                            let mut values = value.into_flat_values();
                            values.reverse();
                            LeafInput::Array { r#type, values }
                        }
                    }
                })
                .collect::<Vec<LeafInput>>(),
            _ => return Err(Error::InvalidStorageValue),
        };
        let storage = DatabaseStorage::new(storage_leaves);

        let synthesizable = ContractSynthesizer {
            inputs: Some(arguments_flat),
            output: &mut result,
            bytecode: self.inner,
            method,
            storage,
            transaction: input.transaction,

            _pd: PhantomData,
        };

        let proof = groth16::create_random_proof(synthesizable, &params, rng)
            .map_err(Error::SynthesisError)?;

        match result {
            None => Err(Error::InternalError(
                "contract hasn't generate outputs".into(),
            )),
            Some(result) => match result {
                Ok(result) => {
                    let output_flat: Vec<BigInt> =
                        result.into_iter().filter_map(|value| value).collect();
                    let output_value =
                        zinc_build::Value::from_flat_values(output_type, &output_flat);

                    Ok((output_value, proof))
                }
                Err(error) => Err(error),
            },
        }
    }
}
