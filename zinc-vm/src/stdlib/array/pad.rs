use crate::core::EvaluationStack;
use crate::error::MalformedBytecode;
use crate::error::Result;
use crate::stdlib::NativeFunction;
use crate::Engine;
use bellman::ConstraintSystem;

pub struct Pad {
    array_length: usize,
}

impl Pad {
    pub fn new(inputs_count: usize) -> Result<Self> {
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

impl<E: Engine> NativeFunction<E> for Pad {
    fn execute<CS: ConstraintSystem<E>>(&self, _cs: CS, stack: &mut EvaluationStack<E>) -> Result {
        let filler = stack.pop()?.value()?;
        let new_length = stack.pop()?.value()?.get_constant_usize()?;

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
