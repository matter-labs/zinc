extern crate franklin_crypto;

use crate::{Operator, RuntimeError, Bytecode, Stack};
use num_bigint::BigInt;
use ff::{PrimeField};
use bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use crate::stack::Primitive;

/// Decodes constant from bytecode and pushes it onto stack.
/// See bytecode specification for details.
pub struct Push;

const MAX_CONSTANT_LENGTH: u8 = 32;

impl<E, CS> Operator<E, CS> for Push where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>,
        bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>
    {
        let len = bytecode.next_byte().ok_or(RuntimeError::InvalidArguments)?;
        let constant = Self::decode_constant(len, bytecode)?;

        let value: E::Fr = E::Fr::from_str(&constant.to_string()).ok_or(RuntimeError::SynthesisError)?;

        match cs.alloc(|| "push", || Ok(value)) {
            Ok(var) => {
                stack.push(Primitive { value: Some(value), variable: var });
                Ok(())
            },
            Err(_) => Err(RuntimeError::SynthesisError)
        }
    }
}

impl Push {
    fn decode_constant(len: u8, bytecode: &mut Bytecode) -> Result<BigInt, RuntimeError> {
        let bytes = bytecode.next_bytes(len as usize).ok_or(RuntimeError::InvalidArguments)?;

        let mut constant = BigInt::from(0);

        for (i, &b) in bytes.iter().enumerate() {
            constant += (b as usize) << (8 * i);
        }

        Ok(constant)
    }
}
