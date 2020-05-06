//!
//! The semantic analyzer scope variable item.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

///
/// The variable item, declared using a `let` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub location: Location,
    pub is_mutable: bool,
    pub r#type: Type,
}

impl Variable {
    pub fn new(location: Location, is_mutable: bool, r#type: Type) -> Self {
        Self {
            location,
            is_mutable,
            r#type,
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_mutable {
            write!(f, "mutable {}", self.r#type)
        } else {
            write!(f, "{}", self.r#type)
        }
    }
}
