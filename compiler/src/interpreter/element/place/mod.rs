//!
//! The interpreter place.
//!

mod error;

pub use self::error::Error;

use std::fmt;

use crate::interpreter::Value;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Place {
    pub identifier: Identifier,
    pub value: Value,
    pub is_mutable: bool,
}

impl Place {
    pub fn new(identifier: Identifier, value: Value, is_mutable: bool) -> Self {
        Self {
            identifier,
            value,
            is_mutable,
        }
    }

    pub fn assign(&mut self, other: Value) -> Result<(), Error> {
        if !self.value.has_the_same_type_as(&other) {
            return Err(Error::AssignmentTypesMismatch(self.value.to_owned(), other));
        }
        self.value = other;
        Ok(())
    }

    pub fn index(&mut self, other: Value) -> Result<Value, Error> {
        let array = match self.value {
            Value::Array(ref array) => array,
            ref value => return Err(Error::IndexingNotArray(value.to_owned())),
        };

        let index = match other {
            Value::Integer(integer) => integer,
            value => return Err(Error::IndexingExpectedInteger(value)),
        };

        let index = index
            .number
            .get_value()
            .expect("Always returns a value")
            .to_string();
        let index =
            usize::from_str_radix(&index[5..index.len() - 1], crate::BASE_HEXADECIMAL as u32)
                .expect("Always valid");
        let element = array
            .get(index)
            .ok_or(Error::IndexOutOfRange(index))?
            .to_owned();

        Ok(element)
    }
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{} = {}",
            if self.is_mutable { "mut " } else { "" },
            self.identifier,
            self.value
        )
    }
}
