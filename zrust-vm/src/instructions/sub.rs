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
    use crate::{Bytecode, OpCode};
    use crate::instructions::utils::testing::{execute_bytecode, assert_stack_value};

    #[test]
    fn test_add() {
        let stack = execute_bytecode(&mut Bytecode::new(&[
            OpCode::Push as u8, 0x01, 0x01,
            OpCode::Push as u8, 0x01, 0x02,
            OpCode::Sub as u8,
        ]));

        assert_eq!(stack.len(), 1);
        assert_stack_value(&stack, 0, "0x01");
    }
}
