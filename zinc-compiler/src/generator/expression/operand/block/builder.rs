//!
//! The generator expression block operand builder.
//!

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::statement::Statement;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    statements: Vec<Statement>,
    expression: Option<GeneratorExpression>,
}

impl Builder {
    pub fn push_statement(&mut self, value: Statement) {
        self.statements.push(value);
    }

    pub fn set_expression(&mut self, value: GeneratorExpression) {
        self.expression = Some(value);
    }

    pub fn finish(self) -> BlockExpression {
        BlockExpression::new(self.statements, self.expression)
    }
}
