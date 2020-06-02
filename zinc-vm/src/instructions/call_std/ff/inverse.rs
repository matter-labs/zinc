//!
//! The `std::ff::inverse` function.
//!

use bellman::ConstraintSystem;

use crate::core::execution_state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::call_std::INativeFunction;
use crate::IEngine;

pub struct Inverse;

impl<E: IEngine> INativeFunction<E> for Inverse {
    fn execute<CS>(&self, cs: CS, stack: &mut EvaluationStack<E>) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let scalar = stack.pop()?.try_into_value()?;
        let inverse = gadgets::arithmetic::field::inverse(cs, &scalar)?;
        stack.push(inverse.into())
    }
}
