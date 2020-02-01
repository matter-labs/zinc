use crate::gadgets::{Gadget, Primitive};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;

/// Reverse array.
pub struct Reverse;

impl<E: Engine> Gadget<E> for Reverse {
    type Input = Vec<Primitive<E>>;
    type Output = Vec<Primitive<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        mut input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        input.reverse();
        Ok(input)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        output
    }
}
