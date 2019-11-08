use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;
use crate::vm_instruction::VMInstruction;
use zrust_bytecode::Rem;
use crate::instructions::utils;

impl<E, CS> VMInstruction<E, CS> for Rem where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let denominator = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let nominator = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let (_q, r) = utils::div_rem(cs, nominator, denominator)?;

        stack.push(r);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_rem() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x10) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x04) }));
        bytecode.push(Box::new(Rem));
        bytecode.push(Box::new(Push { value: BigInt::from(0x9) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x4) }));
        bytecode.push(Box::new(Rem));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 2);
        testing_utils::assert_stack_value(&stack, 0, "0x01");
        testing_utils::assert_stack_value(&stack, 1, "0x00");

        Ok(())
    }
}
