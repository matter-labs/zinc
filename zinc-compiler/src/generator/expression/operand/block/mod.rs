//!
//! The generator expression block operand.
//!

pub mod builder;

use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::statement::Statement;

#[derive(Debug, Clone)]
pub struct Expression {
    statements: Vec<Statement>,
    expression: Option<GeneratorExpression>,
}

impl Expression {
    pub fn new(statements: Vec<Statement>, expression: Option<GeneratorExpression>) -> Self {
        Self {
            statements,
            expression,
        }
    }
}
