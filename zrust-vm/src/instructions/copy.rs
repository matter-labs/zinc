use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;
use num_traits::cast::ToPrimitive;
use crate::vm_instruction::VMInstruction;
use zrust_bytecode::instructions::Copy;

const MAX_CONSTANT_LENGTH: u8 = 4;

impl<E, CS> VMInstruction<E, CS> for Copy where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        _cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let index = self.index.to_u64().ok_or(RuntimeError::InternalError)?;

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

#[cfg(test)]
mod test {
    use crate::{Bytecode, OpCode};
    use crate::instructions::utils::testing::{execute_bytecode, assert_stack_value};

    #[test]
    fn test_add() {
        let stack = execute_bytecode(&mut Bytecode::new(&[
            OpCode::Push as u8, 0x01, 0x01,
            OpCode::Push as u8, 0x01, 0x02,
            OpCode::Push as u8, 0x01, 0x03,
            OpCode::Copy as u8, 0x01, 0x00,
            OpCode::Copy as u8, 0x01, 0x02,
        ]));

        assert_eq!(stack.len(), 5);
        assert_stack_value(&stack, 0, "0x02");
        assert_stack_value(&stack, 1, "0x03");
        assert_stack_value(&stack, 2, "0x03");
        assert_stack_value(&stack, 3, "0x02");
        assert_stack_value(&stack, 4, "0x01");
    }
}
