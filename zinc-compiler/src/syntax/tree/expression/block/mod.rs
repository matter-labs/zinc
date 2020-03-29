//!
//! The block expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub statements: Vec<FunctionLocalStatement>,
    pub expression: Option<Box<ExpressionTree>>,
}

impl Expression {
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
