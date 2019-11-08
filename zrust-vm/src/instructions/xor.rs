use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;
use zrust_bytecode::instructions::Xor;
use crate::vm_instruction::VMInstruction;

impl<E, CS> VMInstruction<E, CS> for Xor where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let left = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let right = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let xor = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                if a.is_zero() == b.is_zero() {
                    Some(E::Fr::zero())
                } else {
                    Some(E::Fr::one())
                }
            }
            _ => None
        };

        let xor_var = cs.alloc(
            || "conjunction",
            || xor.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        // (a + a) * (b) = (a + b - c)
        cs.enforce(
            || "equality",
            |lc| lc + left.variable + left.variable,
            |lc| lc + right.variable,
            |lc| lc + left.variable + right.variable - xor_var
        );

        stack.push(Primitive { value: xor, variable: xor_var });

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
    fn test_xor() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Xor));
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Xor));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Xor));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Xor));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 4);
        testing_utils::assert_stack_value(&stack, 0, "0x00");
        testing_utils::assert_stack_value(&stack, 1, "0x01");
        testing_utils::assert_stack_value(&stack, 2, "0x01");
        testing_utils::assert_stack_value(&stack, 3, "0x00");

        Ok(())
    }
}
