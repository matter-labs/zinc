//!
//! The const statement.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: Type,
    pub expression: Expression,
}

impl Statement {
    pub fn new(
        location: Location,
        identifier: Identifier,
        r#type: Type,
        expression: Expression,
    ) -> Self {
        Self {
            location,
            identifier,
            r#type,
            expression,
        }
    }
}
