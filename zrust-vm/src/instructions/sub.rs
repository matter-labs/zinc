use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;
use crate::vm_instruction::VMInstruction;
use zrust_bytecode::instructions::Sub;

impl<E, CS> VMInstruction<E, CS> for Sub where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let right = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let left = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let diff = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                let mut diff = a;
                diff.sub_assign(&b);
                Some(diff)
            }
            _ => None
        };

        let diff_var = cs.alloc(
            || "diff",
            || diff.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + left.variable - right.variable,
            |lc| lc + CS::one(),
            |lc| lc + diff_var
        );

        stack.push(Primitive { value: diff, variable: diff_var });

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
    fn test_sub() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x02) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Sub));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 1);
        testing_utils::assert_stack_value(&stack, 0, "0x01");

        Ok(())
    }
}
