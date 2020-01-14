//!
//! The semantic analyzer tuple value element.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::semantic::FieldAccessResult;
use crate::semantic::Type;
use crate::semantic::Value;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tuple {
    element_types: Vec<Type>,
}

impl Tuple {
    pub fn new(element_types: Vec<Type>) -> Self {
        Self { element_types }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            element_types: Vec::with_capacity(capacity),
        }
    }

    pub fn slice(&self, index: usize) -> Result<FieldAccessResult, Error> {
        let mut offset = 0;
        let total_size = self.r#type().size();

        if index >= self.element_types.len() {
            return Err(Error::FieldDoesNotExistInTuple(
                index,
                self.r#type().to_string(),
            ));
        }

        let mut tuple_index = 0;
        while tuple_index < index {
            offset += self.element_types[tuple_index].size();
            tuple_index += 1;
        }

        Ok(FieldAccessResult::new(
            offset,
            self.element_types[tuple_index].size(),
            total_size,
            Some(Value::new(self.element_types[tuple_index].to_owned())),
        ))
    }

    pub fn r#type(&self) -> Type {
        Type::new_tuple(self.element_types.to_owned())
    }

    pub fn push(&mut self, r#type: Type) {
        self.element_types.push(r#type);
    }

    pub fn len(&self) -> usize {
        self.element_types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.element_types.is_empty()
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.element_types == other.element_types
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.r#type())
    }
}
