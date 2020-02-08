mod constrained;
pub mod stdlib;
pub mod utils;

use std::fmt::Debug;

use crate::Engine;
use bellman::ConstraintSystem;

pub use constrained::*;

use crate::core::RuntimeError;
use crate::gadgets::utils::dummy_constraint_system::DummyConstraintSystem;
use crate::gadgets::utils::fr_to_bigint;
use franklin_crypto::bellman::Variable;
use num_traits::ToPrimitive;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrimitiveType {
    pub signed: bool,
    pub length: usize,
}

pub trait TypeToString {
    fn type_to_string(&self) -> String;
}

impl TypeToString for Option<PrimitiveType> {
    fn type_to_string(&self) -> String {
        match self {
            None => "field".into(),
            Some(t) => {
                if t.length == 1 {
                    "bool".into()
                } else {
                    format!(
                        "{}{}",
                        if t.signed { "i" } else { "u" },
                        t.length,
                    )
                }
            }
        }
    }
}

impl PrimitiveType {
    pub const BOOLEAN: Self = PrimitiveType {
        signed: false,
        length: 1,
    };
}

/// Primitive is a primitive value that can be stored on the stack and operated by VM's instructions.
#[derive(Clone)]
pub struct Primitive<E: Engine> {
    value: Option<E::Fr>,
    variable: Variable,
    data_type: Option<PrimitiveType>,
}

impl<E: Engine> Primitive<E> {
    pub fn get_data_type(&self) -> Option<PrimitiveType> {
        self.data_type
    }

    pub fn get_constant(&self) -> Result<E::Fr, RuntimeError> {
        self.value.ok_or(RuntimeError::ExpectedConstant)
    }

    pub fn get_constant_usize(&self) -> Result<usize, RuntimeError> {
        let fr = self.get_constant()?;
        let bigint = fr_to_bigint(&fr);
        bigint.to_usize().ok_or_else(|| RuntimeError::ExpectedUsize(bigint))
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
