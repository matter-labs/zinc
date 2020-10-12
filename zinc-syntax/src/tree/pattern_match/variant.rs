//!
//! The match pattern variant.
//!

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;
use crate::tree::literal::boolean::Literal as BooleanLiteral;
use crate::tree::literal::integer::Literal as IntegerLiteral;

///
/// The match pattern variant.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// A boolean refutable literal pattern.
    BooleanLiteral(BooleanLiteral),
    /// An integer refutable literal pattern.
    IntegerLiteral(IntegerLiteral),
    /// A variable irrefutable binding pattern.
    Binding(Identifier),
    /// An expression path refutable pattern, usually points to a constant or enumeration variant.
    Path(ExpressionTree),
    /// A wildcard irrefutable pattern.
    Wildcard,
}

impl Variant {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_boolean_literal(literal: BooleanLiteral) -> Self {
        Self::BooleanLiteral(literal)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_integer_literal(literal: IntegerLiteral) -> Self {
        Self::IntegerLiteral(literal)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_binding(identifier: Identifier) -> Self {
        Self::Binding(identifier)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_path(expression: ExpressionTree) -> Self {
        Self::Path(expression)
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_wildcard() -> Self {
        Self::Wildcard
    }
}
