use crate::{Operator, RuntimeError, Stack, Bytecode};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;

/// Removes top element from the stack.
pub struct Pop;

impl<E, CS> Operator<E, CS> for Pop where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
            &self,
            _cs: &mut CS,
            stack: &mut Stack<E>,
            _bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>
    {
        match stack.pop() {
            Some(_) => Ok(()),
            None => Err(RuntimeError::StackUnderflow),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Bytecode, OpCode};
    use crate::operators::utils::testing::{execute_bytecode, assert_stack_value};

    #[test]
    fn test_pop() {
        let stack = execute_bytecode(&mut Bytecode::new(&[
            OpCode::Push as u8, 0x01, 0x01,
            OpCode::Push as u8, 0x01, 0x02,
            OpCode::Pop as u8,
            OpCode::Push as u8, 0x01, 0x03,
        ]));

        assert_eq!(stack.len(), 2);
        assert_stack_value(&stack, 0, "0x03");
        assert_stack_value(&stack, 1, "0x01");
    }
}
