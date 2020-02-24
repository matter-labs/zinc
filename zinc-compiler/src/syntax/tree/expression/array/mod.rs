//!
//! The array expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression as SyntaxExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<SyntaxExpression>,
    pub size_expression: Option<SyntaxExpression>,
}

impl Expression {
    pub fn new(
        location: Location,
        elements: Vec<SyntaxExpression>,
        size_expression: Option<SyntaxExpression>,
    ) -> Self {
        Self {
            location,
            elements,
            size_expression,
        }
    }
}
