//!
//! The semantic analyzer scope variable item variant.
//!

use std::fmt;

use crate::semantic::element::r#type::Type;

///
/// The variable item, declared using a `let` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub is_mutable: bool,
    pub r#type: Type,
}

impl Variable {
    pub fn new(is_mutable: bool, r#type: Type) -> Self {
        Self { is_mutable, r#type }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_mutable {
            write!(f, "mut {}", self.r#type)
        } else {
            write!(f, "{}", self.r#type)
        }
    }
}
