use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;
use crate::vm_instruction::VMInstruction;
use zrust_bytecode::instructions::Pop;

impl<E, CS> VMInstruction<E, CS> for Pop where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
            &self,
            _cs: &mut CS,
            stack: &mut Stack<E>)
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
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_pop() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x02) }));
        bytecode.push(Box::new(Pop));
        bytecode.push(Box::new(Push { value: BigInt::from(0x03) }));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 2);
        testing_utils::assert_stack_value(&stack, 0, "0x03");
        testing_utils::assert_stack_value(&stack, 1, "0x01");

        Ok(())
    }
}
