use crate::{Operator, RuntimeError, Stack, Bytecode};
use franklin_crypto::bellman::ConstraintSystem;
use bellman::pairing::Engine;
use ff::Field;
use crate::stack::Primitive;

/// Removes two elements from the stack and pushes their sum.
pub struct Mul;

impl<E, CS> Operator<E, CS> for Mul where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>,
        _bytecode: &mut Bytecode)
        -> Result<(), RuntimeError>
    {
        let a = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let b = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let mut a_fr: E::Fr = a.value.ok_or(RuntimeError::SynthesisError)?;
        let b_fr: E::Fr = b.value.ok_or(RuntimeError::SynthesisError)?;

        a_fr.mul_assign(&b_fr);
        let variable = cs.alloc(|| "sum value", || Ok(a_fr)).map_err(|_| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "sum equality",
            |lc| lc + a.variable + b.variable,
            |lc| lc + CS::one(),
            |lc| lc + variable
        );

        stack.push(Primitive { value: Some(a_fr), variable });

        Ok(())
    }
}
