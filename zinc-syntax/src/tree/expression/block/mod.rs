//!
//! The block expression.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::statement::local_fn::Statement as FunctionLocalStatement;

///
/// The block expression.
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    /// The location of the syntax construction.
    pub location: Location,
    /// The function block statements.
    pub statements: Vec<FunctionLocalStatement>,
    /// The optional last statement, which is the result of the block.
    pub expression: Option<Box<ExpressionTree>>,
}

impl Expression {
    ///
    /// Creates a block expression.
    ///
    pub fn new(
        location: Location,
        statements: Vec<FunctionLocalStatement>,
        expression: Option<ExpressionTree>,
    ) -> Self {
        Self {
            location,
            statements,
            expression: expression.map(Box::new),
        }
    }
}
