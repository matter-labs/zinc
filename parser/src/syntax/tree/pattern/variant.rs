//!
//! The pattern variant.
//!

use std::fmt;

use crate::Identifier;
use crate::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Literal(Literal),
    Binding(Identifier),
    Ignoring,
}

impl Variant {
    pub fn new_literal(literal: Literal) -> Self {
        Self::Literal(literal)
    }

    pub fn new_binding(identifier: Identifier) -> Self {
        Self::Binding(identifier)
    }

    pub fn new_ignoring() -> Self {
        Self::Ignoring
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Literal(variant) => write!(f, "{}", variant),
            Self::Binding(variant) => write!(f, "{}", variant),
            Self::Ignoring => write!(f, "_"),
        }
    }
}
