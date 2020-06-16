//!
//! The `std::array::reverse` function call.
//!

use franklin_crypto::bellman::ConstraintSystem;

use crate::core::execution_state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::instructions::call_std::INativeCallable;
use crate::IEngine;

pub struct Reverse {
    array_length: usize,
}

impl Reverse {
    pub fn new(inputs_count: usize) -> Result<Self, RuntimeError> {
        Ok(Self {
            array_length: inputs_count,
        })
    }
}

impl<E: IEngine> INativeCallable<E> for Reverse {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result<(), RuntimeError> {
        let mut array = Vec::with_capacity(self.array_length);

        for _ in 0..self.array_length {
            let value = stack.pop()?;
            array.push(value);
        }

        for value in array {
            stack.push(value)?;
        }

        Ok(())
    }
}
