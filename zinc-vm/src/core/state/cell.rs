use crate::gadgets::Scalar;
use crate::Engine;
use crate::RuntimeError;

#[derive(Debug, Clone)]
pub enum Cell<E: Engine> {
    Value(Scalar<E>),
    //    Address(usize),
}

impl<E: Engine> Cell<E> {
    pub fn value(self) -> Result<Scalar<E>, RuntimeError> {
        match self {
            Cell::Value(value) => Ok(value),
            //            Cell::Address(_) => Err(RuntimeError::UnexpectedNonValueType),
        }
    }
}

impl<E: Engine> From<Scalar<E>> for Cell<E> {
    fn from(scalar: Scalar<E>) -> Self {
        Cell::Value(scalar)
    }
}
