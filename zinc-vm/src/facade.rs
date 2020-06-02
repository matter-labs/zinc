//!
//! The Zinc virtual machine facade.
//!

use std::marker::PhantomData;

use num_bigint::BigInt;

use bellman::groth16;
use bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::groth16::Parameters;
use franklin_crypto::bellman::groth16::Proof;
use franklin_crypto::bellman::groth16::VerifyingKey;
use franklin_crypto::bellman::ConstraintSystem;
use franklin_crypto::circuit::test::TestConstraintSystem;

use zinc_bytecode::Program as BytecodeProgram;
use zinc_bytecode::TemplateValue;

use crate::constraint_systems::debug::DebugConstraintSystem;
use crate::core::circuit::synthesizer::Synthesizer as CircuitSynthesizer;
use crate::core::circuit::Circuit;
use crate::core::contract::synthesizer::Synthesizer as ContractSynthesizer;
use crate::core::contract::Contract;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::error::TypeSizeError;
use crate::error::VerificationError;
use crate::gadgets;
use crate::gadgets::contract::storage::StorageGadget;
use crate::gadgets::contract::Sha256Hasher;
use crate::storage::dummy::DummyStorage;
use crate::IEngine;

pub fn run<E: IEngine>(
    bytecode: BytecodeProgram,
    input: TemplateValue,
) -> Result<TemplateValue, RuntimeError> {
    let mut cs = DebugConstraintSystem::<Bn256>::default();

    let inputs_flat = input.to_flat_values();
    let output = bytecode.output().to_owned();

    match bytecode {
        BytecodeProgram::Circuit(circuit) => {
            let mut vm = Circuit::new(cs, true);

            let mut num_constraints = 0;
            let result = vm.run(
                &BytecodeProgram::Circuit(circuit),
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

            let cs = vm.constraint_system();
            if !cs.is_satisfied() {
                return Err(RuntimeError::UnsatisfiedConstraint);
            }

            let output_flat = result
                .into_iter()
                .map(|v| v.expect("`run` always computes witness"))
                .collect::<Vec<_>>();

            let value =
                TemplateValue::from_flat_values(&output, &output_flat).ok_or_else(|| {
                    TypeSizeError::Output {
                        expected: 0,
                        actual: 0,
                    }
                })?;

            Ok(value)
        }
        BytecodeProgram::Contract(contract) => {
            let storage_fields = contract
                .storage
                .iter()
                .map(|(_name, r#type)| r#type.to_owned())
                .collect();
            let storage = DummyStorage::new(storage_fields);
            let storage_gadget =
                StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
            let mut vm = Contract::new(cs, storage_gadget, true);

            let mut num_constraints = 0;
            let result = vm.run(
                &BytecodeProgram::Contract(contract),
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

            let cs = vm.constraint_system();
            if !cs.is_satisfied() {
                return Err(RuntimeError::UnsatisfiedConstraint);
            }

            let output_flat = result
                .into_iter()
                .map(|v| v.expect("`run` always computes witness"))
                .collect::<Vec<_>>();

            let value =
                TemplateValue::from_flat_values(&output, &output_flat).ok_or_else(|| {
                    TypeSizeError::Output {
                        expected: 0,
                        actual: 0,
                    }
                })?;

            Ok(value)
        }
    }
}

pub fn debug<E: IEngine>(
    bytecode: BytecodeProgram,
    input: TemplateValue,
) -> Result<TemplateValue, RuntimeError> {
    let mut cs = TestConstraintSystem::<Bn256>::new();

    let inputs_flat = input.to_flat_values();
    let output = bytecode.output().to_owned();

    match bytecode {
        BytecodeProgram::Circuit(circuit) => {
            let mut vm = Circuit::new(cs, true);

            let mut num_constraints = 0;
            let result = vm.run(
                &BytecodeProgram::Circuit(circuit),
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

            let cs = vm.constraint_system();

            log::trace!("{}", cs.pretty_print());

            if !cs.is_satisfied() {
                log::error!("Unsatisfied: {}", cs.which_is_unsatisfied().unwrap());
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
                .map(|v| v.expect("`run` always computes witness"))
                .collect::<Vec<_>>();

            let value =
                TemplateValue::from_flat_values(&output, &output_flat).ok_or_else(|| {
                    TypeSizeError::Output {
                        expected: 0,
                        actual: 0,
                    }
                })?;

            Ok(value)
        }
        BytecodeProgram::Contract(contract) => {
            let storage_fields = contract
                .storage
                .iter()
                .map(|(_name, r#type)| r#type.to_owned())
                .collect();

            let storage = DummyStorage::new(storage_fields);
            let storage_gadget =
                StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
            let mut vm = Contract::new(cs, storage_gadget, true);

            let mut num_constraints = 0;
            let result = vm.run(
                &BytecodeProgram::Contract(contract),
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

            let cs = vm.constraint_system();

            log::trace!("{}", cs.pretty_print());

            if !cs.is_satisfied() {
                log::error!("Unsatisfied: {}", cs.which_is_unsatisfied().unwrap());
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
                .map(|v| v.expect("`run` always computes witness"))
                .collect::<Vec<_>>();

            let value =
                TemplateValue::from_flat_values(&output, &output_flat).ok_or_else(|| {
                    TypeSizeError::Output {
                        expected: 0,
                        actual: 0,
                    }
                })?;

            Ok(value)
        }
    }
}

pub fn setup<E: IEngine>(bytecode: BytecodeProgram) -> Result<Parameters<E>, RuntimeError> {
    let rng = &mut rand::thread_rng();
    let mut result = None;

    let params = match bytecode {
        BytecodeProgram::Circuit(circuit) => {
            let synthesizable = CircuitSynthesizer {
                inputs: None,
                output: &mut result,
                bytecode: BytecodeProgram::Circuit(circuit),

                _pd: PhantomData,
            };

            groth16::generate_random_parameters::<E, _, _>(synthesizable, rng)?
        }
        BytecodeProgram::Contract(contract) => {
            let storage_fields = contract
                .storage
                .iter()
                .map(|(_name, r#type)| r#type.to_owned())
                .collect();
            let storage = DummyStorage::new(storage_fields);

            let synthesizable = ContractSynthesizer {
                inputs: None,
                output: &mut result,
                bytecode: BytecodeProgram::Contract(contract),
                storage,

                _pd: PhantomData,
            };

            groth16::generate_random_parameters::<E, _, _>(synthesizable, rng)?
        }
    };

    match result.expect("vm should return either output or error") {
        Ok(_) => Ok(params),
        Err(error) => Err(error),
    }
}

pub fn prove<E: IEngine>(
    bytecode: BytecodeProgram,
    params: Parameters<E>,
    witness: TemplateValue,
) -> Result<(TemplateValue, Proof<E>), RuntimeError> {
    let rng = &mut rand::thread_rng();

    let witness_flat = witness.to_flat_values();
    let output = bytecode.output().to_owned();

    let (result, proof) = {
        let mut result = None;

        let proof = match bytecode {
            BytecodeProgram::Circuit(circuit) => {
                let synthesizable = CircuitSynthesizer {
                    inputs: Some(witness_flat),
                    output: &mut result,
                    bytecode: BytecodeProgram::Circuit(circuit),

                    _pd: PhantomData,
                };

                groth16::create_random_proof(synthesizable, &params, rng)
                    .map_err(RuntimeError::SynthesisError)?
            }
            BytecodeProgram::Contract(contract) => {
                let storage_fields = contract
                    .storage
                    .iter()
                    .map(|(_name, r#type)| r#type.to_owned())
                    .collect();
                let storage = DummyStorage::new(storage_fields);

                let synthesizable = ContractSynthesizer {
                    inputs: Some(witness_flat),
                    output: &mut result,
                    bytecode: BytecodeProgram::Contract(contract),
                    storage,

                    _pd: PhantomData,
                };

                groth16::create_random_proof(synthesizable, &params, rng)
                    .map_err(RuntimeError::SynthesisError)?
            }
        };

        (result, proof)
    };

    match result {
        None => Err(RuntimeError::InternalError(
            "circuit hasn't generate outputs".into(),
        )),
        Some(res) => match res {
            Ok(values) => {
                let output_flat: Vec<BigInt> = values
                    .into_iter()
                    .map(|v| v.expect("`prove` always computes witness"))
                    .collect();

                let value =
                    TemplateValue::from_flat_values(&output, &output_flat).ok_or_else(|| {
                        TypeSizeError::Output {
                            expected: 0,
                            actual: 0,
                        }
                    })?;

                Ok((value, proof))
            }
            Err(err) => Err(err),
        },
    }
}

pub fn verify<E: IEngine>(
    key: VerifyingKey<E>,
    proof: Proof<E>,
    public_input: TemplateValue,
) -> std::result::Result<bool, VerificationError> {
    let public_input_flat = public_input
        .to_flat_values()
        .into_iter()
        .map(|value| {
            gadgets::fr_bigint::bigint_to_fr::<E>(&value)
                .ok_or_else(|| VerificationError::ValueOverflow(value))
        })
        .collect::<std::result::Result<Vec<E::Fr>, VerificationError>>()?;

    let pvk = groth16::prepare_verifying_key(&key);
    let success = groth16::verify_proof(&pvk, &proof, public_input_flat.as_slice())
        .map_err(VerificationError::SynthesisError)?;

    Ok(success)
}
