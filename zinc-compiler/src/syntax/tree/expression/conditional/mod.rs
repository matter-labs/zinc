//!
//! The conditional expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub condition: Box<ExpressionTree>,
    pub main_block: BlockExpression,
    pub else_block: Option<BlockExpression>,
}

impl Expression {
    pub fn new(
        location: Location,
        condition: ExpressionTree,
        main_block: BlockExpression,
        else_block: Option<BlockExpression>,
    ) -> Self {
        Self {
            location,
            condition: Box::new(condition),
            main_block,
            else_block,
        }
    }
}
