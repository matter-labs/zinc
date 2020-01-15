use crate::gadgets::{Gadget, Primitive};
use crate::RuntimeError;
use bellman::ConstraintSystem;
use pairing::Engine;

pub struct Sha256;

impl<E: Engine> Gadget<E> for Sha256 {
    type Input = Vec<Primitive<E>>;
    type Output = Vec<Primitive<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        _cs: CS,
        _input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        unimplemented!()
    }

    fn input_from_vec(_input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        unimplemented!()
    }

    fn output_into_vec(_output: Self::Output) -> Vec<Primitive<E>> {
        unimplemented!()
    }
}
