use std::fmt::Debug;

use bellman::groth16;
use bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::groth16::{Parameters, Proof, VerifyingKey};
use franklin_crypto::bellman::{Circuit, ConstraintSystem, SynthesisError};
use num_bigint::BigInt;

use zinc_bytecode::program::Program;

use crate::constraint_systems::{DebugConstraintSystem, DuplicateRemovingCS};
use crate::core::{VMState, VirtualMachine};
pub use crate::errors::{MalformedBytecode, Result, RuntimeError, TypeSizeError};
use crate::gadgets::contracts::merkle_tree_storage::merkle_tree_hash::Sha256Hasher;
use crate::gadgets::merkle_tree_storage::MerkleTreeStorage;
use crate::gadgets::utils::bigint_to_fr;
use crate::gadgets::StorageGadget;
use crate::storage::dummy::DummyStorage;
use crate::Engine;
use failure::Fail;
use franklin_crypto::circuit::test::TestConstraintSystem;
use std::marker::PhantomData;
use zinc_bytecode::data::values::Value;

struct VMCircuit<'a, E: Engine, S: MerkleTreeStorage<E>> {
    program: &'a Program,
    inputs: Option<&'a [BigInt]>,
    result: &'a mut Option<Result<Vec<Option<BigInt>>>>,
    storage: S,

    _pd: PhantomData<E>,
}

impl<E: Engine, S: MerkleTreeStorage<E>> Circuit<E> for VMCircuit<'_, E, S> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS,
    ) -> std::result::Result<(), SynthesisError> {
        // let cs = LoggingConstraintSystem::new(cs.namespace(|| "logging"));
        let mut cs = DuplicateRemovingCS::new(cs.namespace(|| "duplicates removing"));
        let storage = StorageGadget::<_, _, Sha256Hasher>::new(
            cs.namespace(|| "storage init"),
            self.storage,
        )?;
        let mut vm = VMState::new(cs.namespace(|| "vm"), false, storage);
        *self.result = Some(vm.run(self.program, self.inputs, |_| {}, |_| Ok(())));
        Ok(())
    }
}

pub fn run<E: Engine>(program: &Program, inputs: &Value) -> Result<Value> {
    let mut cs = DebugConstraintSystem::<Bn256>::default();
    let storage = DummyStorage::new(20);
    let storage_gadget =
        StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
    let mut vm = VMState::new(cs, true, storage_gadget);

    let inputs_flat = inputs.to_flat_values();

    let mut num_constraints = 0;
    let result = vm.run(
        program,
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

    let value = Value::from_flat_values(&program.output, &output_flat).ok_or_else(|| {
        TypeSizeError::Output {
            expected: 0,
            actual: 0,
        }
    })?;

    Ok(value)
}

pub fn debug<E: Engine>(program: &Program, inputs: &Value) -> Result<Value> {
    let mut cs = TestConstraintSystem::<Bn256>::new();
    let storage = DummyStorage::new(20);
    let storage_gadget =
        StorageGadget::<_, _, Sha256Hasher>::new(cs.namespace(|| "storage"), storage)?;
    let mut vm = VMState::new(cs, true, storage_gadget);

    let inputs_flat = inputs.to_flat_values();

    let mut num_constraints = 0;
    let result = vm.run(
        program,
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
        log::error!("unsatisfied: {}", cs.which_is_unsatisfied().unwrap());
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

    let value = Value::from_flat_values(&program.output, &output_flat).ok_or_else(|| {
        TypeSizeError::Output {
            expected: 0,
            actual: 0,
        }
    })?;

    Ok(value)
}

pub fn setup<E: Engine>(program: &Program) -> Result<Parameters<E>> {
    let rng = &mut rand::thread_rng();
    let mut result = None;

    let storage: DummyStorage<E> = unimplemented!(); // todo: add setup storage

    let circuit = VMCircuit {
        program,
        inputs: None,
        result: &mut result,
        storage,
        _pd: PhantomData,
    };

    let params = groth16::generate_random_parameters::<E, _, _>(circuit, rng)?;

    match result.expect("vm should return either output or error") {
        Ok(_) => Ok(params),
        Err(error) => Err(error),
    }
}

pub fn prove<E: Engine>(
    program: &Program,
    params: &Parameters<E>,
    witness: &Value,
) -> Result<(Value, Proof<E>)> {
    let rng = &mut rand::thread_rng();

    let witness_flat = witness.to_flat_values();

    let storage = DummyStorage::new(20);

    let (result, proof) = {
        let mut result = None;
        let circuit = VMCircuit {
            program,
            inputs: Some(&witness_flat),
            result: &mut result,
            storage,
            _pd: PhantomData,
        };

        let proof = groth16::create_random_proof(circuit, params, rng)
            .map_err(RuntimeError::SynthesisError)?;

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
                    Value::from_flat_values(&program.output, &output_flat).ok_or_else(|| {
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

#[derive(Debug, Fail)]
pub enum VerificationError {
    #[fail(display = "value overflow: value {} is not in the field", _0)]
    ValueOverflow(BigInt),

    #[fail(display = "failed to synthesize circuit: {}", _0)]
    SynthesisError(SynthesisError),
}

pub fn verify<E: Engine>(
    key: &VerifyingKey<E>,
    proof: &Proof<E>,
    public_input: &Value,
) -> std::result::Result<bool, VerificationError> {
    let public_input_flat = public_input
        .to_flat_values()
        .into_iter()
        .map(|value| {
            bigint_to_fr::<E>(&value).ok_or_else(|| VerificationError::ValueOverflow(value))
        })
        .collect::<std::result::Result<Vec<E::Fr>, VerificationError>>()?;

    let pvk = groth16::prepare_verifying_key(&key);
    let success = groth16::verify_proof(&pvk, proof, public_input_flat.as_slice())
        .map_err(VerificationError::SynthesisError)?;

    Ok(success)
}
