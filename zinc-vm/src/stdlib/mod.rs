pub mod array;
pub mod bits;
pub mod crypto;
pub mod ff;

use bellman::ConstraintSystem;

use crate::core::EvaluationStack;
use crate::error::Result;
use crate::Engine;

pub trait NativeFunction<E: Engine> {
    fn execute<CS: ConstraintSystem<E>>(&self, cs: CS, stack: &mut EvaluationStack<E>) -> Result;
}
