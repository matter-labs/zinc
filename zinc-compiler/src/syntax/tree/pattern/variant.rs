//!
//! The pattern variant.
//!

use crate::syntax::BooleanLiteral;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    Binding(Identifier),
    Ignoring,
}

impl Variant {
    pub fn new_boolean_literal(literal: BooleanLiteral) -> Self {
        Self::BooleanLiteral(literal)
    }

    pub fn new_integer_literal(literal: IntegerLiteral) -> Self {
        Self::IntegerLiteral(literal)
    }

    pub fn new_binding(identifier: Identifier) -> Self {
        Self::Binding(identifier)
    }

    pub fn new_ignoring() -> Self {
        Self::Ignoring
    }
}
