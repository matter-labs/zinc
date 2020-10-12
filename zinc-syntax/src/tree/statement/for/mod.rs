//!
//! The `for` statement.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;

///
/// The `for` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The loop index variable identifier.
    pub index_identifier: Identifier,
    /// The loop index bounds range expression.
    pub bounds_expression: ExpressionTree,
    /// The optional loop `while` condition expression.
    pub while_condition: Option<ExpressionTree>,
    /// The loop block.
    pub block: BlockExpression,
}

impl Statement {
    ///
    /// Creates a `for` statement.
    ///
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
