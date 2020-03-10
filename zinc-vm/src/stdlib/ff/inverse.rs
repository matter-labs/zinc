use bellman::ConstraintSystem;

use crate::core::EvaluationStack;
use crate::gadgets::stdlib::NativeFunction;
use crate::{gadgets, Engine, Result};

pub struct Inverse;

impl<E: Engine> NativeFunction<E> for Inverse {
    fn execute<CS>(&self, cs: CS, stack: &mut EvaluationStack<E>) -> Result
    where
        CS: ConstraintSystem<E>,
    {
        let scalar = stack.pop()?.value()?;
        let inverse = gadgets::arithmetic::inverse(cs, &scalar)?;
        stack.push(inverse.into())
    }
}
