use crate::{Operator, RuntimeError, Stack, Bytecode};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;

/// Removes two elements from the stack and pushes their production.
pub struct Mul;

impl<E, CS> Operator<E, CS> for Mul where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>,
        _bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>
    {
        let left = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let right = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let mut prod = match (left.value, right.value) {
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
