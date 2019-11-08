use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;
use zrust_bytecode::instructions::Not;
use crate::vm_instruction::VMInstruction;

impl<E, CS> VMInstruction<E, CS> for Not where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let element = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let inverse = match element.value {
            Some(value) => {
                let mut inverse = E::Fr::one();
                inverse.sub_assign(&value);
                Some(inverse)
            }
            _ => None
        };

        let inverse_var = cs.alloc(
            || "inverse",
            || inverse.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + element.variable + inverse_var,
            |lc| lc + CS::one(),
            |lc| lc + CS::one()
        );

        stack.push(Primitive { value: inverse, variable: inverse_var });

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
    fn test_not() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Not));
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Not));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 2);
        testing_utils::assert_stack_value(&stack, 0, "0x01");
        testing_utils::assert_stack_value(&stack, 1, "0x00");

        Ok(())
    }
}
