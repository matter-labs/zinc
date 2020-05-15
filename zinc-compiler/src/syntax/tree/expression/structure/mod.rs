//!
//! The structure or identifier expression.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub identifier: Identifier,
    pub is_structure: bool,
    pub fields: Vec<(Identifier, ExpressionTree)>,
}

impl Expression {
    pub fn new(
        location: Location,
        identifier: Identifier,
        is_structure: bool,
        fields: Vec<(Identifier, ExpressionTree)>,
    ) -> Self {
        Self {
            location,
            identifier,
            is_structure,
            fields,
        }
    }
}
