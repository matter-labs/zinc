use crate::element::utils::bigint_to_fr;
use crate::element::ConstrainedElementOperator;
use crate::vm::VirtualMachine;
use crate::RuntimeError;
use bellman::groth16;
use bellman::pairing::bn256::Bn256;
use bellman::pairing::Engine;
use franklin_crypto::bellman::groth16::{Parameters, PreparedVerifyingKey, Proof};
use franklin_crypto::bellman::{Circuit, ConstraintSystem, SynthesisError};
use franklin_crypto::circuit::test::TestConstraintSystem;
use num_bigint::BigInt;
use rand::ThreadRng;
use std::fmt::Debug;
use zrust_bytecode::Instruction;

struct VMCircuit<'a, 'b, 'c> {
    code: &'a [Instruction],
    inputs: Option<&'b [BigInt]>,
    result: &'c mut Option<Result<Vec<Option<BigInt>>, RuntimeError>>,
}

impl<E: Engine + Debug> Circuit<E> for VMCircuit<'_, '_, '_> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let mut vm = VirtualMachine::new(ConstrainedElementOperator::new(cs));
        *self.result = Some(vm.run(self.code, self.inputs));
        Ok(())
    }
}

fn generate_parameters<E: Engine + Debug>(
    code: &[Instruction],
) -> Result<Parameters<E>, RuntimeError> {
    let rng = &mut rand::thread_rng();
    let mut result = None;
    let circuit = VMCircuit {
        code,
        inputs: None,
        result: &mut result,
    };

    groth16::generate_random_parameters::<E, VMCircuit, ThreadRng>(circuit, rng)
        .map_err(|_| RuntimeError::SynthesisError)
}

pub fn exec<E: Engine>(
    code: &[Instruction],
    inputs: &[BigInt],
) -> Result<Vec<Option<BigInt>>, RuntimeError> {
    let cs = TestConstraintSystem::<Bn256>::new();
    let mut vm = VirtualMachine::new(ConstrainedElementOperator::new(cs));
    vm.run(code, Some(inputs))
}

pub fn gen_key<E: Engine + Debug>(
    code: &[Instruction],
) -> Result<PreparedVerifyingKey<E>, RuntimeError> {
    let params = generate_parameters::<E>(code)?;
    Ok(groth16::prepare_verifying_key(&params.vk))
}

pub fn prove<E: Engine + Debug>(
    code: &[Instruction],
    inputs: &[BigInt],
) -> Result<Proof<E>, RuntimeError> {
    let rng = &mut rand::thread_rng();
    let params = generate_parameters::<E>(code)?;

    let (result, proof) = {
        let mut result = None;
        let circuit = VMCircuit {
            code,
            inputs: Some(inputs),
            result: &mut result,
        };

        let proof = groth16::create_random_proof(circuit, &params, rng)
            .map_err(|_| RuntimeError::SynthesisError)?;

        (result, proof)
    };

    match result {
        None => Err(RuntimeError::InternalError),
        Some(res) => match res {
            Ok(_) => Ok(proof),
            Err(err) => Err(err),
        },
    }
}

#[derive(Debug)]
pub enum VerificationError {
    InputFormatError,
    SynthesisError,
}

pub fn verify<E: Engine + Debug>(
    key: &PreparedVerifyingKey<E>,
    proof: &Proof<E>,
    outputs: &[BigInt],
) -> Result<bool, VerificationError> {
    let mut inputs = Vec::new();
    for v in outputs.iter() {
        let fr = bigint_to_fr::<E>(v).ok_or(VerificationError::InputFormatError)?;
        inputs.push(fr);
    }

    let success = groth16::verify_proof(key, proof, inputs.as_slice())
        .map_err(|_| VerificationError::SynthesisError)?;

    Ok(success)
}
