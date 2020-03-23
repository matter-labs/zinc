//!
//! The generator expression group operand.
//!

pub mod builder;

use crate::generator::expression::Expression as GeneratorExpression;

#[derive(Debug, Clone)]
pub struct Expression {
    expressions: Vec<GeneratorExpression>,
}

impl Expression {
    pub fn new(expressions: Vec<GeneratorExpression>) -> Self {
        Self { expressions }
    }
}
