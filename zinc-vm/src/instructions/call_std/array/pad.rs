//!
//! The `std::array::pad` function call.
//!

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::evaluation_stack::EvaluationStack;
use crate::error::MalformedBytecode;
use crate::error::RuntimeError;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct Pad {
    array_length: usize,
}

impl Pad {
    pub fn new(inputs_count: usize) -> Result<Self, RuntimeError> {
        inputs_count
            .checked_sub(2)
            .map(|array_length| Self { array_length })
            .ok_or_else(|| {
                MalformedBytecode::InvalidArguments(
                    "array::pad expects at least 3 arguments".into(),
                )
                .into()
            })
    }
}

impl<E: IEngine> INativeCallable<E> for Pad {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result<(), RuntimeError> {
        let filler = stack.pop()?.try_into_value()?;
        let new_length = stack.pop()?.try_into_value()?.get_constant_usize()?;

        if new_length < self.array_length {
            return Err(MalformedBytecode::InvalidArguments(
                "array::pad: new length can't be smaller".into(),
            )
            .into());
        }

        for _ in 0..(new_length - self.array_length) {
            stack.push(filler.clone().into())?;
        }

        Ok(())
    }
}
