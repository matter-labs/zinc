//!
//! The loop statement.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub index_identifier: Identifier,
    pub bounds_expression: ExpressionTree,
    pub while_condition: Option<ExpressionTree>,
    pub block: BlockExpression,
}

impl Statement {
    pub fn new(
        location: Location,
        index_identifier: Identifier,
        bounds_expression: ExpressionTree,
        while_condition: Option<ExpressionTree>,
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
