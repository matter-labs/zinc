extern crate franklin_crypto;

use crate::{Operator, RuntimeError, Bytecode, Stack};
use ff::PrimeField;
use bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use crate::stack::Primitive;
use crate::operators::utils;

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
        if len < 1 || len > MAX_CONSTANT_LENGTH {
            return Err(RuntimeError::InvalidArguments);
        }
        let constant = utils::decode_constant(len, bytecode)?;

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
