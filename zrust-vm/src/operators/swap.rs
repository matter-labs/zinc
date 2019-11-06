use crate::{Operator, RuntimeError, Stack, Bytecode};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;
use crate::operators::utils;
use num_traits::cast::ToPrimitive;

/// Swaps n-th element form the stack with top element.
#[derive(Debug)]
pub struct Swap;

const MAX_CONSTANT_LENGTH: u8 = 4;

impl<E, CS> Operator<E, CS> for Swap where E: Engine, CS: ConstraintSystem<E> {
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

        match (stack.get(index as usize), stack.top()) {
            (Some(p), Some(t)) => {
                stack.set(index as usize, *t);
                stack.set(0, p);
                Ok(())
            },
            _ => {
                Err(RuntimeError::StackUnderflow)
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Bytecode, OpCode};
    use crate::operators::utils::testing::{execute_bytecode, assert_stack_value};

    #[test]
    fn test_add() {
        let stack = execute_bytecode(&mut Bytecode::new(&[
            OpCode::Push as u8, 0x01, 0x01,
            OpCode::Push as u8, 0x01, 0x02,
            OpCode::Push as u8, 0x01, 0x03,
            OpCode::Swap as u8, 0x01, 0x02,
        ]));

        assert_eq!(stack.len(), 3);
        assert_stack_value(&stack, 0, "0x01");
        assert_stack_value(&stack, 1, "0x02");
        assert_stack_value(&stack, 2, "0x03");
    }
}
