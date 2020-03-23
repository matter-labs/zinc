use crate::core::EvaluationStack;
use crate::stdlib::NativeFunction;
use crate::{Engine, Result};
use bellman::ConstraintSystem;

pub struct Reverse {
    array_length: usize,
}

impl Reverse {
    pub fn new(inputs_count: usize) -> Result<Self> {
        Ok(Self {
            array_length: inputs_count,
        })
    }
}

impl<E: Engine> NativeFunction<E> for Reverse {
    fn execute<CS: ConstraintSystem<E>>(&self, _cs: CS, stack: &mut EvaluationStack<E>) -> Result {
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
