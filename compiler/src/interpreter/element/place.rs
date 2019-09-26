//!
//! The interpreter place.
//!

use std::fmt;

use crate::interpreter::Value;
use crate::interpreter::ValueError;
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

    pub fn assign(&mut self, other: Value) -> Result<(), ValueError> {
        if !self.value.has_the_same_type_as(&other) {
            return Err(ValueError::OperandTypesMismatch(
                self.value.to_owned(),
                other,
            ));
        }
        self.value = other;
        Ok(())
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
