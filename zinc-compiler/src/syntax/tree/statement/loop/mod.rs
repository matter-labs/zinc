//!
//! The loop statement.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Expression;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop {
    pub location: Location,
    pub index_identifier: Identifier,
    pub bounds_expression: Expression,
    pub while_condition: Option<Expression>,
    pub block: BlockExpression,
}

impl Loop {
    pub fn new(
        location: Location,
        index_identifier: Identifier,
        bounds_expression: Expression,
        while_condition: Option<Expression>,
        block: BlockExpression,
    ) -> Self {
        Self {
            location,
            index_identifier,
            bounds_expression,
            while_condition,
            block,
        }
    }
}
