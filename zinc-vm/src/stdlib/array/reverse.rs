//!
//! The `std::array::reverse` function.
//!

use bellman::ConstraintSystem;

use crate::core::state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::stdlib::NativeFunction;
use crate::Engine;

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

impl<E: Engine> NativeFunction<E> for Reverse {
    fn execute<CS: ConstraintSystem<E>>(
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
