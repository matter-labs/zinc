use crate::gadgets::{Gadget, Primitive};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;

pub struct ArrayPad;

impl<E: Engine> Gadget<E> for ArrayPad {
    type Input = (Primitive<E>, Primitive<E>, Vec<Primitive<E>>);
    type Output = Vec<Primitive<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let (value, len_value, mut array) = input;
        let len = len_value.get_constant_usize()?;

        if len < array.len() {
            return Err(RuntimeError::InvalidArguments(format!(
                "ArrayPad: new length ({}) can't be less than old length ({})",
                len,
                array.len()
            )));
        }

        array.resize(len, value);

        Ok(array)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        if input.len() < 2 {
            return Err(RuntimeError::InvalidArguments(format!(
                "ArrayPad expected at least 2 arguments, got {}",
                input.len()
            )));
        }

        Ok((input[0].clone(), input[1].clone(), Vec::from(&input[2..])))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        output
    }
}
