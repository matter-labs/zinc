//!
//! The tuple expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression as SyntaxExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<SyntaxExpression>,
}

impl Expression {
    pub fn new(location: Location, elements: Vec<SyntaxExpression>) -> Self {
        Self { location, elements }
    }
}
