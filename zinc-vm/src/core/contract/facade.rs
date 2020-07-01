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

use zinc_bytecode::Contract as BytecodeContract;
use zinc_bytecode::TemplateValue;
use zinc_const::UnitTestExitCode;

use crate::constraint_systems::debug::DebugCS;
use crate::core::contract::storage::dummy::Storage as DummyStorage;
use crate::core::contract::storage::setup::Storage as SetupStorage;
use crate::core::contract::synthesizer::Synthesizer as ContractSynthesizer;
use crate::core::contract::Contract;
use crate::core::facade::IFacade;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::error::TypeSizeError;
use crate::gadgets::contract::merkle_tree::hasher::sha256::Hasher as Sha256Hasher;
use crate::gadgets::contract::storage::StorageGadget;
use crate::IEngine;

impl IFacade for BytecodeContract {
    fn run<E: IEngine>(self, witness: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        let mut cs = DebugCS::<Bn256>::default();

        let inputs_flat = witness.into_flat_values();
        let output_type = self.output.to_owned();

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = DummyStorage::new(storage_fields);
        let storage_gadget =
            StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
        let mut contract = Contract::new(cs, storage_gadget, true);

        let mut num_constraints = 0;
        let result = contract.run(
            self,
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

        let cs = contract.constraint_system();
        if !cs.is_satisfied() {
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let output_flat = result
            .into_iter()
            .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
            .collect::<Vec<_>>();

        let value =
            TemplateValue::new_from_flat_values(output_type, &output_flat).ok_or_else(|| {
                TypeSizeError::Output {
                    expected: 0,
                    actual: 0,
                }
            })?;

        Ok(value)
    }

    fn debug<E: IEngine>(self, input: TemplateValue) -> Result<TemplateValue, RuntimeError> {
        let mut cs = TestConstraintSystem::<Bn256>::new();

        let inputs_flat = input.into_flat_values();
        let output_type = self.output.to_owned();

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = DummyStorage::new(storage_fields);
        let storage_gadget =
            StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
        let mut contract = Contract::new(cs, storage_gadget, true);

        let mut num_constraints = 0;
        let result = contract.run(
            self,
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

        let cs = contract.constraint_system();

        log::debug!("{}", cs.pretty_print());

        if !cs.is_satisfied() {
            log::error!(
                "Unsatisfied: {}",
                cs.which_is_unsatisfied()
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS)
            );
            return Err(RuntimeError::UnsatisfiedConstraint);
        }

        let unconstrained = cs.find_unconstrained();
        if !unconstrained.is_empty() {
            log::error!("Unconstrained: {}", unconstrained);
            return Err(RuntimeError::InternalError(
                "Generated unconstrained variables".into(),
            ));
        }

        let output_flat = result
            .into_iter()
            .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
            .collect::<Vec<_>>();

        let value =
            TemplateValue::new_from_flat_values(output_type, &output_flat).ok_or_else(|| {
                TypeSizeError::Output {
                    expected: 0,
                    actual: 0,
                }
            })?;

        Ok(value)
    }

    fn test<E: IEngine>(mut self) -> Result<UnitTestExitCode, RuntimeError> {
        let unit_test = self
            .unit_test
            .take()
            .ok_or(RuntimeError::UnitTestDataMissing)?;

        if unit_test.is_ignored {
            println!("test {} ... {}", unit_test.name, "ignore".yellow());
            return Ok(UnitTestExitCode::Ignored);
        }

        let mut cs = TestConstraintSystem::<Bn256>::new();

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = DummyStorage::new(storage_fields);
        let storage_gadget =
            StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;

        let mut contract = Contract::new(cs, storage_gadget, true);

        let result = contract.run(self, Some(&[]), |_| {}, |_| Ok(()));
        let code = match result {
            Ok(_) if unit_test.should_panic => {
                println!(
                    "test {} ... {} (should have failed)",
                    unit_test.name,
                    "error".bright_red()
                );
                UnitTestExitCode::Failed
            }
            Err(_) if unit_test.should_panic => {
                println!("test {} ... {} (failed)", unit_test.name, "ok".green());
                UnitTestExitCode::Passed
            }

            Ok(_) => {
                println!("test {} ... {}", unit_test.name, "ok".green());
                UnitTestExitCode::Passed
            }
            Err(_) => {
                println!("test {} ... {}", unit_test.name, "error".bright_red());
                UnitTestExitCode::Failed
            }
        };

        Ok(code)
    }

    fn setup<E: IEngine>(self) -> Result<Parameters<E>, RuntimeError> {
        let rng = &mut rand::thread_rng();
        let mut result = None;

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = SetupStorage::new(storage_fields);

        let synthesizable = ContractSynthesizer {
            inputs: None,
            output: &mut result,
            bytecode: self,
            storage,

            _pd: PhantomData,
        };

        let params = groth16::generate_random_parameters::<E, _, _>(synthesizable, rng)?;

        match result.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS) {
            Ok(_) => Ok(params),
            Err(error) => Err(error),
        }
    }

    fn prove<E: IEngine>(
        self,
        params: Parameters<E>,
        witness: TemplateValue,
    ) -> Result<(TemplateValue, Proof<E>), RuntimeError> {
        let mut result = None;
        let rng = &mut rand::thread_rng();

        let inputs_flat = witness.into_flat_values();
        let output_type = self.output.to_owned();

        let storage_fields = self
            .storage
            .iter()
            .map(|(_name, r#type)| r#type.to_owned())
            .collect();
        let storage = DummyStorage::new(storage_fields);

        let synthesizable = ContractSynthesizer {
            inputs: Some(inputs_flat),
            output: &mut result,
            bytecode: self,
            storage,

            _pd: PhantomData,
        };

        let proof = groth16::create_random_proof(synthesizable, &params, rng)
            .map_err(RuntimeError::SynthesisError)?;

        match result {
            None => Err(RuntimeError::InternalError(
                "circuit hasn't generate outputs".into(),
            )),
            Some(result) => match result {
                Ok(values) => {
                    let output_flat: Vec<BigInt> = values
                        .into_iter()
                        .map(|v| v.expect(zinc_const::panic::VALUE_ALWAYS_EXISTS))
                        .collect();

                    let value = TemplateValue::new_from_flat_values(output_type, &output_flat)
                        .ok_or_else(|| TypeSizeError::Output {
                            expected: 0,
                            actual: 0,
                        })?;

                    Ok((value, proof))
                }
                Err(error) => Err(error),
            },
        }
    }
}
