mod constrained;
mod scalar;
pub mod stdlib;
pub mod utils;
pub use scalar::*;
mod tmp_lt;

use crate::Engine;
use bellman::ConstraintSystem;

pub use constrained::*;

use crate::core::RuntimeError;
use crate::gadgets::utils::dummy_constraint_system::DummyConstraintSystem;
use franklin_crypto::bellman::Variable;
use num_traits::ToPrimitive;

/// Primitive is a primitive value that can be stored on the stack and operated by VM's instructions.
#[derive(Clone)]
pub struct Primitive<E: Engine> {
    value: Option<E::Fr>,
    variable: Variable,
    scalar_type: ScalarType,
}

impl<E: Engine> Primitive<E> {
    pub fn get_type(&self) -> ScalarType {
        self.scalar_type
    }

    pub fn get_constant(&self) -> Result<E::Fr, RuntimeError> {
        self.value.ok_or(RuntimeError::ExpectedConstant)
    }

    pub fn get_constant_usize(&self) -> Result<usize, RuntimeError> {
        let fr = self.get_constant()?;
        let bigint = utils::fr_to_bigint(&fr, false);
        bigint
            .to_usize()
            .ok_or_else(|| RuntimeError::ExpectedUsize(bigint))
    }

    pub fn as_field(&self) -> Self {
        Self {
            value: self.value,
            variable: self.variable,
            scalar_type: ScalarType::Field,
        }
    }

    pub fn is_signed(&self) -> bool {
        self.scalar_type.is_signed()
    }
}

pub trait Gadget<E: Engine> {
    type Input;
    type Output;

    /// Synthesize circuit for the function.
    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError>;

    /// Calculate function's result without synthesizing a circuit.
    fn calculate(&self, input: Self::Input) -> Result<Self::Output, RuntimeError> {
        let cs = DummyConstraintSystem;
        self.synthesize(cs, input)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError>;
    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>>;

    fn synthesize_vec<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        input: &[Primitive<E>],
    ) -> Result<Vec<Primitive<E>>, RuntimeError> {
        let input = Self::input_from_vec(input)?;
        let output = self.synthesize(cs, input)?;
        Ok(Self::output_into_vec(output))
    }
}

pub use constrained::Gadgets;
