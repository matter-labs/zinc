use crate::{Operator, RuntimeError, Stack, Bytecode};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;
use crate::operators::utils;
use num_traits::cast::ToPrimitive;

/// Copies n-th element form the stack on top of the stack.
pub struct Copy;

const MAX_CONSTANT_LENGTH: u8 = 4;

impl<E, CS> Operator<E, CS> for Copy where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        _cs: &mut CS,
        stack: &mut Stack<E>,
        bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>
    {
        let len = bytecode.next_byte().ok_or(RuntimeError::StackUnderflow)?;
        if len > MAX_CONSTANT_LENGTH {
            return Err(RuntimeError::InvalidArguments);
        }
        let index_bigint = utils::decode_constant(len, bytecode)?;
        let index = index_bigint.to_u64().ok_or(RuntimeError::InternalError)?;

        match stack.get(index as usize) {
            Some(p) => {
                stack.push(p);
                Ok(())
            },
            None => {
                Err(RuntimeError::StackUnderflow)
            },
        }
    }
}
