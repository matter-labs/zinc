//!
//! The VM state cell.
//!

use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::IEngine;

#[derive(Debug, Clone)]
pub enum Cell<E: IEngine> {
    Value(Scalar<E>),
}

impl<E: IEngine> Cell<E> {
    pub fn try_into_value(self) -> Result<Scalar<E>, Error> {
        match self {
            Cell::Value(value) => Ok(value),
        }
    }
}

impl<E: IEngine> From<Scalar<E>> for Cell<E> {
    fn from(scalar: Scalar<E>) -> Self {
        Cell::Value(scalar)
    }
}
