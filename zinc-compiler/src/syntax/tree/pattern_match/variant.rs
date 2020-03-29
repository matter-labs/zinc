//!
//! The match pattern variant.
//!

use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    Binding(Identifier),
    Path(ExpressionTree),
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

    pub fn new_path(expression: ExpressionTree) -> Self {
        Self::Path(expression)
    }

    pub fn new_wildcard() -> Self {
        Self::Wildcard
    }
}
