//!
//! The `std` module tools.
//!

pub mod array;
pub mod convert;
pub mod crypto;
pub mod ff;

use bellman::ConstraintSystem;

use crate::core::state::evaluation_stack::EvaluationStack;
use crate::error::RuntimeError;
use crate::Engine;

pub trait NativeFunction<E: Engine> {
    fn execute<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result<(), RuntimeError>;
}
