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
    use crate::{Bytecode, OpCode};
    use crate::instructions::utils::testing::{execute_bytecode, assert_stack_value};

    #[test]
    fn test_add() {
        let stack = execute_bytecode(&mut Bytecode::new(&[
            OpCode::Push as u8, 0x01, 0x03,
            OpCode::Push as u8, 0x01, 0x04,
            OpCode::Mul as u8,
        ]));

        assert_eq!(stack.len(), 1);
        assert_stack_value(&stack, 0, "0x0C");
    }
}
