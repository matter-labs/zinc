//!
//! The semantic analyzer array element value.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::semantic::IndexAccessResult;
use crate::semantic::Type;
use crate::semantic::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    r#type: Type,
    size: usize,
}

impl Default for Array {
    fn default() -> Self {
        Self {
            r#type: Type::Unit,
            size: 0,
        }
    }
}

impl Array {
    pub fn new(r#type: Type, size: usize) -> Self {
        Self { r#type, size }
    }

    pub fn r#type(&self) -> Type {
        Type::new_array(self.r#type.to_owned(), self.size)
    }

    pub fn slice(&self) -> IndexAccessResult {
        IndexAccessResult::new(
            self.r#type.size(),
            self.r#type().size(),
            Some(Value::new(self.r#type.to_owned())),
        )
    }

    pub fn extend(&mut self, r#type: Type, count: usize) -> Result<(), Error> {
        if self.size == 0 {
            self.r#type = r#type;
        } else if r#type != self.r#type {
            return Err(Error::InvalidType(
                r#type.to_string(),
                self.r#type.to_string(),
            ));
        }
        self.size += count;

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.len() == other.len() && self.r#type == other.r#type
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.r#type())
    }
}
