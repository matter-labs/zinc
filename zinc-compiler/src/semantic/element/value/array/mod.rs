//!
//! The semantic analyzer array element value.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::semantic::Type;

#[derive(Clone, PartialEq)]
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
        Type::new_array(self.r#type.clone(), self.size)
    }

    pub fn push(&mut self, r#type: Type) -> Result<(), Error> {
        if self.size == 0 {
            self.r#type = r#type;
        } else if r#type != self.r#type {
            return Err(Error::PushingInvalidType(
                r#type.to_string(),
                self.r#type.to_string(),
            ));
        }
        self.size += 1;

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

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {}]", self.r#type, self.size,)
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
