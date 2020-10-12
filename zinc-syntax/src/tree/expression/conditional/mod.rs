//!
//! The conditional expression.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The conditional expression.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// The location of the syntax construction.
    pub location: Location,
    /// The condition expression.
    pub condition: Box<ExpressionTree>,
    /// The main conditional block expression.
    pub main_block: BlockExpression,
    /// The `else` conditional block expression.
    pub else_block: Option<BlockExpression>,
}

impl Expression {
    ///
    /// Creates a conditional expression.
    ///
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
