use crate::primitive::Primitive;
use crate::RuntimeError;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell<P: Primitive> {
    Value(P),
    Address(usize),
}

impl<P: Primitive> Cell<P> {
    pub fn value(self) -> Result<P, RuntimeError> {
        match self {
            Cell::Value(value) => Ok(value),
            Cell::Address(_) => Err(RuntimeError::InternalError("...".into())),
        }
    }
}
