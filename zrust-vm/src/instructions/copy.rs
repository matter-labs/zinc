use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;
use num_traits::cast::ToPrimitive;
use crate::vm_instruction::VMInstruction;
use zrust_bytecode::instructions::Copy;

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
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_copy() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x02) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x03) }));
        bytecode.push(Box::new(Copy { index: 0 }));
        bytecode.push(Box::new(Copy { index: 2 }));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 5);
        testing_utils::assert_stack_value(&stack, 0, "0x02");
        testing_utils::assert_stack_value(&stack, 1, "0x03");
        testing_utils::assert_stack_value(&stack, 2, "0x03");
        testing_utils::assert_stack_value(&stack, 3, "0x02");
        testing_utils::assert_stack_value(&stack, 4, "0x01");

        Ok(())
    }
}
