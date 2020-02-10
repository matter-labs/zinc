use std::fmt::Debug;

use bellman::groth16;
use bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::groth16::{Parameters, Proof, VerifyingKey};
use franklin_crypto::bellman::{Circuit, ConstraintSystem, SynthesisError};
use num_bigint::BigInt;
use rand::ThreadRng;

use zinc_bytecode::program::Program;

use crate::core::VirtualMachine;
use crate::debug_constraint_system::DebugConstraintSystem;
pub use crate::errors::{MalformedBytecode, RuntimeError, TypeSizeError};
use crate::gadgets::utils::bigint_to_fr;
use crate::Engine;
use zinc_bytecode::data::values::Value;

struct VMCircuit<'a> {
    program: &'a Program,
    inputs: Option<&'a [BigInt]>,
    result: &'a mut Option<Result<Vec<Option<BigInt>>, RuntimeError>>,
}

impl<E: Engine> Circuit<E> for VMCircuit<'_> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let mut vm = VirtualMachine::new(cs, false);
        *self.result = Some(vm.run(self.program, self.inputs, |_| {}, |_| Ok(())));
        Ok(())
    }
}

pub fn run<E: Engine>(program: &Program, inputs: &[BigInt]) -> Result<Vec<BigInt>, RuntimeError> {
    let cs = DebugConstraintSystem::<Bn256>::default();
    let mut vm = VirtualMachine::new(cs, true);

    let mut num_constraints = 0;
    let result = vm.run(
        program,
        Some(inputs),
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

    //    let unconstrained = cs.find_unconstrained();
    //    if !unconstrained.is_empty() {
    //        log::error!("Unconstrained: {}", unconstrained);
    //        return Err(RuntimeError::InternalError(
    //            "Generated unconstrained variables".into(),
    //        ));
    //    }

    // TODO: Remove unwrap
    Ok(result.into_iter().map(|v| v.unwrap()).collect())
}

pub fn setup<E: Engine>(program: &Program) -> Result<Parameters<E>, RuntimeError> {
    let rng = &mut rand::thread_rng();
    let mut result = None;
    let circuit = VMCircuit {
        program,
        inputs: None,
        result: &mut result,
    };

    let params = groth16::generate_random_parameters::<E, VMCircuit, ThreadRng>(circuit, rng)?;
    Ok(params)
}

pub fn prove<E: Engine>(
    program: &Program,
    params: &Parameters<E>,
    witness: &Value,
) -> Result<(Value, Proof<E>), RuntimeError> {
    let rng = &mut rand::thread_rng();

    let witness_flat = witness.to_flat_values();

    let (result, proof) = {
        let mut result = None;
        let circuit = VMCircuit {
            program,
            inputs: Some(&witness_flat),
            result: &mut result,
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
                let output_flat: Vec<BigInt> = values.into_iter().map(|v| v.unwrap()).collect();
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

#[derive(Debug)]
pub enum VerificationError {
    InputFormatError,
    SynthesisError(SynthesisError),
}

pub fn verify<E: Engine>(
    key: &VerifyingKey<E>,
    proof: &Proof<E>,
    pub_inputs: &[BigInt],
) -> Result<bool, VerificationError> {
    let mut pub_inputs_fr = Vec::new();
    for v in pub_inputs.iter() {
        let fr = bigint_to_fr::<E>(v).ok_or(VerificationError::InputFormatError)?;
        pub_inputs_fr.push(fr);
    }

    let pvk = groth16::prepare_verifying_key(&key);
    let success = groth16::verify_proof(&pvk, proof, pub_inputs_fr.as_slice())
        .map_err(VerificationError::SynthesisError)?;

    Ok(success)
}
