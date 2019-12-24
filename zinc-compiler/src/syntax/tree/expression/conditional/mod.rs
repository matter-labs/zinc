//!
//! The conditional expression.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::BlockExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub condition: Box<syntax::Expression>,
    pub main_block: BlockExpression,
    pub else_block: Option<BlockExpression>,
}

impl Expression {
    pub fn new(
        location: Location,
        condition: syntax::Expression,
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
