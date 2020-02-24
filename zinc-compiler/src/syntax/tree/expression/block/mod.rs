//!
//! The block expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression as SyntaxExpression;
use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub statements: Vec<FunctionLocalStatement>,
    pub expression: Option<Box<SyntaxExpression>>,
}

impl Expression {
    pub fn new(
        location: Location,
        statements: Vec<FunctionLocalStatement>,
        expression: Option<SyntaxExpression>,
    ) -> Self {
        Self {
            location,
            statements,
            expression: expression.map(Box::new),
        }
    }
}
