//!
//! The `std::ff::invert` function call.
//!

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::ExecutionState;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct Inverse;

impl<E: IEngine> INativeCallable<E> for Inverse {
    fn call<CS>(&self, cs: CS, state: &mut ExecutionState<E>) -> Result<(), RuntimeError>
    where
        CS: ConstraintSystem<E>,
    {
        let scalar = state.evaluation_stack.pop()?.try_into_value()?;
        let inverse = gadgets::arithmetic::field::inverse(cs, &scalar)?;
        state.evaluation_stack.push(inverse.into())
    }
}
