//!
//! The match pattern variant.
//!

use crate::syntax::BooleanLiteral;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    Binding(Identifier),
    Path(Expression),
    Wildcard,
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

    pub fn new_path(expression: Expression) -> Self {
        Self::Path(expression)
    }

    pub fn new_wildcard() -> Self {
        Self::Wildcard
    }
}
