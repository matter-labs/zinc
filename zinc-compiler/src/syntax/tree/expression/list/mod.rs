//!
//! The list expression.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub elements: Vec<ExpressionTree>,
}

impl Expression {
    pub fn new(location: Location, elements: Vec<ExpressionTree>) -> Self {
        Self { location, elements }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }
}
