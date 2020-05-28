//!
//! The VM state cell.
//!

use crate::error::RuntimeError;
use crate::gadgets::scalar::Scalar;
use crate::Engine;

#[derive(Debug, Clone)]
pub enum Cell<E: Engine> {
    Value(Scalar<E>),
}

impl<E: Engine> Cell<E> {
    pub fn value(self) -> Result<Scalar<E>, RuntimeError> {
        match self {
            Cell::Value(value) => Ok(value),
        }
    }
}

impl<E: Engine> From<Scalar<E>> for Cell<E> {
    fn from(scalar: Scalar<E>) -> Self {
        Cell::Value(scalar)
    }
}
