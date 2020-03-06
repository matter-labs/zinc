use crate::gadgets::{Gadget, Scalar};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;

/// Reverse array.
pub struct Reverse;

impl<E: Engine> Gadget<E> for Reverse {
    type Input = Vec<Scalar<E>>;
    type Output = Vec<Scalar<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        mut input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        input.reverse();
        Ok(input)
    }

    fn input_from_vec(input: &[Scalar<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Scalar<E>> {
        output
    }
}
