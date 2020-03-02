//!
//! The structure or identifier expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression as SyntaxExpression;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub identifier: Identifier,
    pub is_struct: bool,
    pub fields: Vec<(Identifier, SyntaxExpression)>,
}

impl Expression {
    pub fn new(
        location: Location,
        identifier: Identifier,
        is_struct: bool,
        fields: Vec<(Identifier, SyntaxExpression)>,
    ) -> Self {
        Self {
            location,
            identifier,
            is_struct,
            fields,
        }
    }
}
