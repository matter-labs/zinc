//!
//! The structure or identifier expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub identifier: Identifier,
    pub is_struct: bool,
    pub fields: Vec<(Identifier, ExpressionTree)>,
}

impl Expression {
    pub fn new(
        location: Location,
        identifier: Identifier,
        is_struct: bool,
        fields: Vec<(Identifier, ExpressionTree)>,
    ) -> Self {
        Self {
            location,
            identifier,
            is_struct,
            fields,
        }
    }
}
