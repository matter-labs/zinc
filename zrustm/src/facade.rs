use zrust_bytecode::Instruction;
use num_bigint::BigInt;
use crate::RuntimeError;
use franklin_crypto::circuit::test::TestConstraintSystem;
use crate::vm::VirtualMachine;
use crate::element::{ConstrainedElementOperator};
use bellman::pairing::Engine;
use bellman::pairing::bn256::Bn256;
use franklin_crypto::bellman::{Circuit, SynthesisError, ConstraintSystem};
use bellman::groth16;
use franklin_crypto::bellman::groth16::{Parameters, Proof, PreparedVerifyingKey};
use std::fmt::Debug;
use rand::ThreadRng;

struct VMCircuit<'a, 'b, 'c> {
    code: &'a [Instruction],
    inputs: &'b [BigInt],
    result: &'c mut Option<Result<Vec<Option<BigInt>>, RuntimeError>>,
}

impl<E: Engine + Debug> Circuit<E> for VMCircuit<'_, '_, '_> {
    fn synthesize<CS: ConstraintSystem<E>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let mut vm = VirtualMachine::new(ConstrainedElementOperator::new(cs));
        *self.result = Some(vm.run(self.code, self.inputs));
        Ok(())
    }
}

fn generate_parameters<E: Engine + Debug>(code: &[Instruction], inputs: &[BigInt])
    -> Result<Parameters<E>, RuntimeError>
{
    let rng = &mut rand::thread_rng();
    let mut result = None;
    let circuit = VMCircuit {
        code,
        inputs,
        result: &mut result
    };

    groth16::generate_random_parameters::<E, VMCircuit, ThreadRng>(circuit, rng)
        .map_err(|_| RuntimeError::SynthesisError)
}

pub fn exec<E: Engine>(code: &[Instruction], inputs: &[BigInt])
    -> Result<Vec<Option<BigInt>>, RuntimeError>
{
    let cs = TestConstraintSystem::<Bn256>::new();
    let mut vm = VirtualMachine::new(ConstrainedElementOperator::new(cs));
    vm.run(code, inputs)
}

pub fn gen_key<E: Engine + Debug>(code: &[Instruction], inputs: &[BigInt]) -> Result<PreparedVerifyingKey<E>, RuntimeError>
{
    let params = generate_parameters::<E>(code, inputs)?;
    Ok(groth16::prepare_verifying_key(&params.vk))
}

pub fn gen_proof<E: Engine + Debug>(code: &[Instruction], inputs: &[BigInt])
    -> Result<Proof<E>, RuntimeError>
{
    let rng = &mut rand::thread_rng();
    let params = generate_parameters::<E>(code, inputs)?;

    let (result, proof) = {
        let mut result = None;
        let circuit = VMCircuit {
            code,
            inputs,
            result: &mut result
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
