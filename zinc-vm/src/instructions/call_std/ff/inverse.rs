//!
//! The `std::ff::inverse` function call.
//!

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct Inverse;

impl<E: IEngine> INativeCallable<E> for Inverse {
    fn call<CS>(&self, cs: CS, stack: &mut EvaluationStack<E>) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let scalar = stack.pop()?.try_into_value()?;
        let inverse = gadgets::arithmetic::field::inverse(cs, &scalar)?;
        stack.push(inverse.into())
    }
}
