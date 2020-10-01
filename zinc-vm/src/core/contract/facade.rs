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
use zinc_build::ContractFieldValue;
use zinc_build::Type as BuildType;
use zinc_build::Value as BuildValue;
use zinc_const::UnitTestExitCode;

// use crate::constraint_systems::main::Main as MainCS;
use crate::constraint_systems::constant::Constant as ConstantCS;
use crate::core::contract::output::Output as ContractOutput;
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
    ) -> Result<ContractOutput, RuntimeError> {
        let mut cs = ConstantCS {};

        let method = self
            .inner
            .methods
            .get(method_name.as_str())
            .cloned()
            .ok_or(RuntimeError::MethodNotFound {
                found: method_name.clone(),
            })?;

        let inputs_flat = input.into_flat_values();
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
        let storage_values = match storage {
            BuildValue::Contract(fields) => fields
                .into_iter()
                .map(|field| {
                    let mut values = field.value.into_flat_values();
                    values.reverse();
                    values
                })
                .collect::<Vec<Vec<BigInt>>>(),
            _ => return Err(RuntimeError::InvalidStorageValue),
        };
        let storage = DatabaseStorage::<Bn256>::new(storage_types.clone(), storage_values);
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
                log::trace!("Constraints: {}", num);
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

        let output_value: Vec<BigInt> = result.into_iter().filter_map(|value| value).collect();
        let output_value = BuildValue::from_flat_values(output_type, &output_value);

        let storage_value = BuildValue::Contract(
            state
                .storage
                .into_inner()
                .into_values()
                .into_iter()
                .zip(storage_fields)
                .enumerate()
                .map(|(index, (mut values, field))| {
                    values.reverse();

                    ContractFieldValue::new(
                        field.name,
                        BuildValue::from_flat_values(
                            storage_types
                                .get(index)
                                .cloned()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            values.as_slice(),
                        ),
                        field.is_public,
                        field.is_external,
                    )
                })
                .collect::<Vec<ContractFieldValue>>(),
        );

        let transfers = state.execution_state.transfers;

        Ok(ContractOutput::new(output_value, storage_value, transfers))
    }

    pub fn debug<E: IEngine>(
        self,
        input: BuildValue,
        storage: BuildValue,
        method_name: String,
    ) -> Result<ContractOutput, RuntimeError> {
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
        let storage_values = match storage {
            BuildValue::Contract(fields) => fields
                .into_iter()
                .map(|field| {
                    let mut values = field.value.into_flat_values();
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
                log::trace!("Constraints: {}", num);
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
            log::trace!("{}", cs.pretty_print());
            log::error!(
                "Unsatisfied: {}",
                cs.which_is_unsatisfied()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
            );
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let output_value: Vec<BigInt> = result.into_iter().filter_map(|value| value).collect();
        let output_value = BuildValue::from_flat_values(output_type, &output_value);

        let storage_value = BuildValue::Contract(
            state
                .storage
                .into_inner()
                .into_values()
                .into_iter()
                .zip(storage_fields)
                .enumerate()
                .map(|(index, (mut values, field))| {
                    values.reverse();

                    ContractFieldValue::new(
                        field.name,
                        BuildValue::from_flat_values(
                            storage_types
                                .get(index)
                                .cloned()
                                .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS),
                            values.as_slice(),
                        ),
                        field.is_public,
                        field.is_external,
                    )
                })
                .collect::<Vec<ContractFieldValue>>(),
        );

        let transfers = state.execution_state.transfers;

        Ok(ContractOutput::new(output_value, storage_value, transfers))
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
                .map(|field| field.r#type)
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
            .map(|field| field.r#type.to_owned())
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
        let output_type = if method.is_mutable {
            method.output.into_mutable_method_output()
        } else {
            method.output
        };

        let mut storage_types = Vec::with_capacity(self.inner.storage.len());
        for field in self.inner.storage.iter() {
            storage_types.push(field.r#type.to_owned());
        }
        let storage_values = match storage {
            BuildValue::Contract(fields) => fields
                .into_iter()
                .map(|field| {
                    let mut values = field.value.into_flat_values();
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
                Ok(result) => {
                    let output_flat: Vec<BigInt> =
                        result.into_iter().filter_map(|value| value).collect();
                    let output_value = BuildValue::from_flat_values(output_type, &output_flat);

                    Ok((output_value, proof))
                }
                Err(error) => Err(error),
            },
        }
    }
}
