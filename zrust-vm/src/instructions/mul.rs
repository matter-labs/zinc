use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;
use crate::vm_instruction::VMInstruction;
use zrust_bytecode::instructions::Mul;

impl<E, CS> VMInstruction<E, CS> for Mul where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let left = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let right = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let prod = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                let mut prod = a;
                prod.mul_assign(&b);
                Some(prod)
            }
            _ => None
        };

        let prod_var = cs.alloc(
            || "production",
            || prod.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + left.variable,
            |lc| lc + right.variable,
            |lc| lc + prod_var
        );

        stack.push(Primitive { value: prod, variable: prod_var });

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
    fn test_mul() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x03) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x04) }));
        bytecode.push(Box::new(Mul));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 1);
        testing_utils::assert_stack_value(&stack, 0, "0x0C");

        Ok(())
    }
}
