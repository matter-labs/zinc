use crate::gadgets::Primitive;
use crate::Engine;
use crate::RuntimeError;

#[derive(Debug, Clone)]
pub enum Cell<E: Engine> {
    Value(Primitive<E>),
    //    Address(usize),
}

impl<E: Engine> Cell<E> {
    pub fn value(self) -> Result<Primitive<E>, RuntimeError> {
        match self {
            Cell::Value(value) => Ok(value),
            //            Cell::Address(_) => Err(RuntimeError::UnexpectedNonValueType),
        }
    }
}
