use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;
use zrust_bytecode::instructions::Add;
use crate::vm_instruction::VMInstruction;

impl<E, CS> VMInstruction<E, CS> for Add where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let left = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let right = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let sum = match (left.value, right.value) {
            (Some(a), Some(b)) => {
                let mut sum = a;
                sum.add_assign(&b);
                Some(sum)
            }
            _ => None
        };

        let sum_var = cs.alloc(
            || "sum",
            || sum.ok_or(SynthesisError::AssignmentMissing)
        ).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + left.variable + right.variable,
            |lc| lc + CS::one(),
            |lc| lc + sum_var
        );

        stack.push(Primitive { value: sum, variable: sum_var });

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
            OpCode::Push as u8, 0x01, 0x01,
            OpCode::Push as u8, 0x01, 0x02,
            OpCode::Add as u8,
        ]));

        assert_eq!(stack.len(), 1);
        assert_stack_value(&stack, 0, "0x03");
    }
}
