use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;
use zrust_bytecode::instructions::Or;
use crate::vm_instruction::VMInstruction;

impl<E, CS> VMInstruction<E, CS> for Or where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let left = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let right = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let or_value = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                if a.is_zero() && b.is_zero() {
                    Some(E::Fr::zero())
                } else {
                    Some(E::Fr::one())
                }
            }
            _ => None
        };

        let or_variable = cs.alloc(
            || "or",
            || or_value.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + CS::one() - left.variable,
            |lc| lc + CS::one() - right.variable,
            |lc| lc + CS::one() - or_variable
        );

        stack.push(Primitive { value: or_value, variable: or_variable });

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
    fn test_or() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Or));
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Or));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Or));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Or));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 4);
        testing_utils::assert_stack_value(&stack, 0, "0x01");
        testing_utils::assert_stack_value(&stack, 1, "0x01");
        testing_utils::assert_stack_value(&stack, 2, "0x01");
        testing_utils::assert_stack_value(&stack, 3, "0x00");

        Ok(())
    }
}
