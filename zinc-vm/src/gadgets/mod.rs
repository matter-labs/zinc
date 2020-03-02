pub mod arithmetic;
pub mod arrays;
pub mod auto_const;
pub mod boolean;
pub mod comparison;
mod conditional_select;
pub mod types;

pub use arithmetic::*;
pub use arrays::*;
pub use boolean::*;
pub use comparison::*;
pub use conditional_select::*;
pub use types::*;

mod misc;
mod scalar;
pub mod stdlib;
pub mod utils;
pub use scalar::*;

use crate::Engine;
use bellman::ConstraintSystem;

pub use misc::*;

use crate::core::RuntimeError;

pub trait Gadget<E: Engine> {
    type Input;
    type Output;

    /// Synthesize circuit for the function.
    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError>;

    fn input_from_vec(input: &[Scalar<E>]) -> Result<Self::Input, RuntimeError>;
    fn output_into_vec(output: Self::Output) -> Vec<Scalar<E>>;

    fn synthesize_vec<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        input: &[Scalar<E>],
    ) -> Result<Vec<Scalar<E>>, RuntimeError> {
        let input = Self::input_from_vec(input)?;
        let output = self.synthesize(cs, input)?;
        Ok(Self::output_into_vec(output))
    }
}

pub use misc::Gadgets;
