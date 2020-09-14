//!
//! The `std::array::truncate` function call.
//!

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::ExecutionState;
use crate::error::MalformedBytecode;
use crate::error::RuntimeError;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct Truncate {
    array_length: usize,
}

impl Truncate {
    pub fn new(inputs_count: usize) -> Result<Self, RuntimeError> {
        inputs_count
            .checked_sub(1)
            .map(|array_length| Self { array_length })
            .ok_or_else(|| {
                MalformedBytecode::InvalidArguments(
                    "array::truncate expects at least 2 arguments".into(),
                )
                .into()
            })
    }
}

impl<E: IEngine> INativeCallable<E> for Truncate {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        state: &mut ExecutionState<E>,
    ) -> Result<(), RuntimeError> {
        let new_length = state
            .evaluation_stack
            .pop()?
            .try_into_value()?
            .get_constant_usize()?;

        if new_length > self.array_length {
            return Err(MalformedBytecode::InvalidArguments(
                "array::truncate: new length can't be smaller".into(),
            )
            .into());
        }

        for _ in 0..(self.array_length - new_length) {
            state.evaluation_stack.pop()?;
        }

        Ok(())
    }
}
