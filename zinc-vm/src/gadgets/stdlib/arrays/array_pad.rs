use crate::errors::MalformedBytecode;
use crate::gadgets::{Gadget, Primitive};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;

pub struct ArrayPad;

pub struct Input<E: Engine> {
    len: Primitive<E>,
    fill_value: Primitive<E>,
    array: Vec<Primitive<E>>,
}

impl<E: Engine> Gadget<E> for ArrayPad {
    type Input = Input<E>;
    type Output = Vec<Primitive<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let Input {
            fill_value,
            len,
            mut array,
        } = input;
        let len = len.get_constant_usize()?;

        if len < array.len() {
            return Err(MalformedBytecode::InvalidArguments(format!(
                "ArrayPad: new length ({}) can't be less than old length ({})",
                len,
                array.len()
            ))
            .into());
        }

        array.resize(len, fill_value);

        Ok(array)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        if input.len() < 2 {
            return Err(MalformedBytecode::InvalidArguments(format!(
                "ArrayPad expected at least 2 arguments, got {}",
                input.len()
            ))
            .into());
        }

        let (len, args) = input
            .split_last()
            .ok_or_else(|| MalformedBytecode::InvalidArguments(
                "pad expects at least 3 arguments".into(),
            ))?;
        let (fill_value, args) = args
            .split_last()
            .ok_or_else(|| MalformedBytecode::InvalidArguments(
                "pad expects at least 3 arguments".into(),
            ))?;

        Ok(Input {
            len: len.clone(),
            fill_value: fill_value.clone(),
            array: args.into(),
        })
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        output
    }
}
