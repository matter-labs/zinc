//!
//! The virtual machine contract facade.
//!

use std::marker::PhantomData;

use colored::Colorize;
use num_bigint::BigInt;

use franklin_crypto::bellman::groth16;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_build::Contract as BuildContract;
use zinc_build::Type as BuildType;
use zinc_build::Value as BuildValue;
use zinc_const::UnitTestExitCode;

use crate::constraint_systems::main::Main as MainCS;
use crate::core::contract::storage::database::Storage as DatabaseStorage;
use crate::core::contract::storage::setup::Storage as SetupStorage;
use crate::core::contract::synthesizer::Synthesizer as ContractSynthesizer;
use crate::core::contract::State as ContractState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
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

    pub fn run<E: IEngine>(
        self,
        input: BuildValue,
        storage: BuildValue,
        method_name: String,
    ) -> Result<(BuildValue, BuildValue), RuntimeError> {
        let mut cs = MainCS::<Bn256>::new();

        let method = self
            .inner
            .methods
            .get(method_name.as_str())
            .cloned()
            .ok_or(RuntimeError::MethodNotFound {
                found: method_name.clone(),
            })?;

        let inputs_flat = input.into_flat_values();
        let output_type = method.output.into_contract_metadata();

        let mut storage_names = Vec::with_capacity(self.inner.storage.len());
        let mut storage_types = Vec::with_capacity(self.inner.storage.len());
        for (name, r#type) in self.inner.storage.clone().into_iter() {
            storage_names.push(name);
            storage_types.push(r#type);
        }
        let storage_values = match storage {
            BuildValue::Contract(fields) => fields
                .into_iter()
                .map(|(_name, value)| {
                    let mut values = value.into_flat_values();
                    values.reverse();
                    values
                })
                .collect::<Vec<Vec<BigInt>>>(),
            _ => return Err(RuntimeError::InvalidStorageValue),
        };
        let storage = DatabaseStorage::new(storage_types.clone(), storage_values);
        let storage_gadget =
            StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;

        let mut state = ContractState::new(cs, storage_gadget, false);

        let mut num_constraints = 0;
        let result = state.run(
            self.inner,
            method_name,
            Some(&inputs_flat),
            |cs| {
                let num = cs.num_constraints() - num_constraints;
                num_constraints += num;
                log::debug!("Constraints: {}", num);
            },
            |cs| {
                if !cs.is_satisfied() {
                    return Err(RuntimeError::UnsatisfiedConstraint);
                }

                Ok(())
            },
        )?;

        let cs = state.constraint_system();
        if !cs.is_satisfied() {
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let output_value = result
            .into_iter()
            .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
            .collect::<Vec<_>>();
        let output_value = BuildValue::from_flat_values(output_type, &output_value);

        let storage_value = BuildValue::Contract(
            state
                .into_storage()
                .into_values()
                .into_iter()
                .zip(storage_names)
                .enumerate()
                .map(|(index, (mut values, name))| {
                    values.reverse();

                    (
                        name,
                        BuildValue::from_flat_values(
                            storage_types
                                .get(index)
                                .cloned()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            values.as_slice(),
                        ),
                    )
                })
                .collect::<Vec<(String, BuildValue)>>(),
        );

        Ok((output_value, storage_value))
    }

    pub fn debug<E: IEngine>(
        self,
        input: BuildValue,
        storage: BuildValue,
        method_name: String,
    ) -> Result<(BuildValue, BuildValue), RuntimeError> {
        let mut cs = TestConstraintSystem::<Bn256>::new();

        let method = self
            .inner
            .methods
            .get(method_name.as_str())
            .cloned()
            .ok_or(RuntimeError::MethodNotFound {
                found: method_name.clone(),
            })?;

        let inputs_flat = input.into_flat_values();
        let output_type = method.output.into_contract_metadata();

        let mut storage_names = Vec::with_capacity(self.inner.storage.len());
        let mut storage_types = Vec::with_capacity(self.inner.storage.len());
        for (name, r#type) in self.inner.storage.clone().into_iter() {
            storage_names.push(name);
            storage_types.push(r#type);
        }
        let storage_values = match storage {
            BuildValue::Contract(fields) => fields
                .into_iter()
                .map(|(_name, value)| {
                    let mut values = value.into_flat_values();
                    values.reverse();
                    values
                })
                .collect::<Vec<Vec<BigInt>>>(),
            _ => return Err(RuntimeError::InvalidStorageValue),
        };
        let storage = DatabaseStorage::new(storage_types.clone(), storage_values);
        let storage_gadget =
            StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;

        let mut state = ContractState::new(cs, storage_gadget, false);

        let mut num_constraints = 0;
        let result = state.run(
            self.inner,
            method_name,
            Some(&inputs_flat),
            |cs| {
                let num = cs.num_constraints() - num_constraints;
                num_constraints += num;
                log::debug!("Constraints: {}", num);
            },
            |cs| {
                if !cs.is_satisfied() {
                    return Err(RuntimeError::UnsatisfiedConstraint);
                }

                Ok(())
            },
        )?;

        let cs = state.constraint_system();
        if !cs.is_satisfied() {
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let output_value = result
            .into_iter()
            .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
            .collect::<Vec<_>>();
        let output_value = BuildValue::from_flat_values(output_type, &output_value);

        let storage_value = BuildValue::Contract(
            state
                .into_storage()
                .into_values()
                .into_iter()
                .zip(storage_names)
                .enumerate()
                .map(|(index, (mut values, name))| {
                    values.reverse();

                    (
                        name,
                        BuildValue::from_flat_values(
                            storage_types
                                .get(index)
                                .cloned()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            values.as_slice(),
                        ),
                    )
                })
                .collect::<Vec<(String, BuildValue)>>(),
        );

        Ok((output_value, storage_value))
    }

    pub fn test<E: IEngine>(self) -> Result<UnitTestExitCode, RuntimeError> {
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
                .map(|(_name, r#type)| r#type)
                .collect::<Vec<BuildType>>();
            let storage = SetupStorage::new(storage_types);
            let storage_gadget =
                StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;

            let mut state = ContractState::new(cs, storage_gadget, true);

            let result = state.run(
                self.inner.clone(),
                name.clone(),
                Some(&[]),
                |_| {},
                |_| Ok(()),
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
                Err(_) => {
                    println!("test {} ... {}", name, "error".bright_red());
                    exit_code = UnitTestExitCode::Failed;
                }
            };
        }

        Ok(exit_code)
    }

    pub fn setup<E: IEngine>(self, method_name: String) -> Result<Parameters<E>, RuntimeError> {
        let rng = &mut rand::thread_rng();
        let mut result = None;

        let storage_fields = self
            .inner
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = SetupStorage::new(storage_fields);

        let synthesizable = ContractSynthesizer {
            inputs: None,
            output: &mut result,
            bytecode: self.inner,
            method_name,
            storage,

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
        input: BuildValue,
        storage: BuildValue,
        method_name: String,
    ) -> Result<(BuildValue, Proof<E>), RuntimeError> {
        let method = self
            .inner
            .methods
            .get(method_name.as_str())
            .cloned()
            .ok_or(RuntimeError::MethodNotFound {
                found: method_name.clone(),
            })?;

        let mut result = None;
        let rng = &mut rand::thread_rng();

        let inputs_flat = input.into_flat_values();
        let output_type = method.output.into_contract_metadata();

        let mut storage_names = Vec::with_capacity(self.inner.storage.len());
        let mut storage_types = Vec::with_capacity(self.inner.storage.len());
        for (name, r#type) in self.inner.storage.clone().into_iter() {
            storage_names.push(name);
            storage_types.push(r#type);
        }
        let storage_values = match storage {
            BuildValue::Contract(fields) => fields
                .into_iter()
                .map(|(_name, value)| {
                    let mut values = value.into_flat_values();
                    values.reverse();
                    values
                })
                .collect::<Vec<Vec<BigInt>>>(),
            _ => return Err(RuntimeError::InvalidStorageValue),
        };
        let storage = DatabaseStorage::new(storage_types, storage_values);

        let synthesizable = ContractSynthesizer {
            inputs: Some(inputs_flat),
            output: &mut result,
            bytecode: self.inner,
            method_name,
            storage,

            _pd: PhantomData,
        };

        let proof = groth16::create_random_proof(synthesizable, &params, rng)
            .map_err(RuntimeError::SynthesisError)?;

        match result {
            None => Err(RuntimeError::InternalError(
                "contract hasn't generate outputs".into(),
            )),
            Some(result) => match result {
                Ok(values) => {
                    let output_flat: Vec<BigInt> = values
                        .into_iter()
                        .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
                        .collect();

                    let value = BuildValue::from_flat_values(output_type, &output_flat);

                    Ok((value, proof))
                }
                Err(error) => Err(error),
            },
        }
    }
}
