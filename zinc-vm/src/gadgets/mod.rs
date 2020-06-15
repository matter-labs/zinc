pub mod arithmetic;
pub mod array;
pub mod assert;
pub mod auto_const;
pub mod bitwise;
pub mod comparison;
pub mod contract;
pub mod logical;
pub mod output;
pub mod scalar;
pub mod select;
pub mod witness;

use franklin_crypto::bellman::ConstraintSystem;

use crate::error::RuntimeError;
use crate::IEngine;

use self::scalar::Scalar;

pub trait IGadget<E: IEngine> {
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
