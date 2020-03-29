//!
//! The let statement.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub is_mutable: bool,
    pub r#type: Option<Type>,
    pub expression: ExpressionTree,
}

impl Statement {
    pub fn new(
        location: Location,
        identifier: Identifier,
        is_mutable: bool,
        r#type: Option<Type>,
        expression: ExpressionTree,
    ) -> Self {
        Self {
            location,
            identifier,
            is_mutable,
            r#type,
            expression,
        }
    }
}
