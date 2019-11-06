extern crate franklin_crypto;

use crate::{RuntimeError, Stack};
use ff::PrimeField;
use bellman::pairing::Engine;
use franklin_crypto::bellman::ConstraintSystem;
use crate::stack::Primitive;
use crate::vm_instruction::VMInstruction;
use zrust_bytecode::instructions::Push;

impl<E, CS> VMInstruction<E, CS> for Push where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let value: E::Fr = E::Fr::from_str(&self.value.to_string()).ok_or(RuntimeError::SynthesisError)?;

        match cs.alloc(|| "push", || Ok(value)) {
            Ok(var) => {
                stack.push(Primitive { value: Some(value), variable: var });
                Ok(())
            },
            Err(_) => Err(RuntimeError::SynthesisError)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;

    #[test]
    fn test_push() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x42) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0xABCD) }));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 3);
        testing_utils::assert_stack_value(&stack, 0, "0xABCD");
        testing_utils::assert_stack_value(&stack, 1, "0x42");
        testing_utils::assert_stack_value(&stack, 2, "0x00");

        Ok(())
    }
}
