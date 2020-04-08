pub mod array;
pub mod bits;
pub mod crypto;
pub mod ff;

use crate::core::EvaluationStack;
use crate::{Engine, Result};
use bellman::ConstraintSystem;

pub trait NativeFunction<E: Engine> {
    fn execute<CS: ConstraintSystem<E>>(&self, cs: CS, stack: &mut EvaluationStack<E>) -> Result;
}
