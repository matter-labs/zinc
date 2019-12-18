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
    pub id: String,
    pub index_identifier: Identifier,
    pub range_start_expression: Expression,
    pub range_end_expression: Expression,
    pub is_range_inclusive: bool,
    pub while_condition: Option<Expression>,
    pub block: BlockExpression,
}

impl Loop {
    pub fn new(
        location: Location,
        index_identifier: Identifier,
        range_start_expression: Expression,
        range_end_expression: Expression,
        is_range_inclusive: bool,
        while_condition: Option<Expression>,
        block: BlockExpression,
    ) -> Self {
        let id = format!("L{}", location.line);

        Self {
            location,
            id,
            index_identifier,
            range_start_expression,
            range_end_expression,
            is_range_inclusive,
            while_condition,
            block,
        }
    }
}
