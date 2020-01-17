use crate::gadgets::Primitive;
use crate::RuntimeError;
use crate::ZincEngine;

#[derive(Debug, Clone)]
pub enum Cell<E: ZincEngine> {
    Value(Primitive<E>),
    //    Address(usize),
}

impl<E: ZincEngine> Cell<E> {
    pub fn value(self) -> Result<Primitive<E>, RuntimeError> {
        match self {
            Cell::Value(value) => Ok(value),
            //            Cell::Address(_) => Err(RuntimeError::UnexpectedNonValueType),
        }
    }
}
