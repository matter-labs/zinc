//!
//! The generator expression group operand builder.
//!

use crate::generator::expression::operand::group::Expression as GroupExpression;
use crate::generator::expression::Expression as GeneratorExpression;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    expressions: Vec<GeneratorExpression>,
}

impl Builder {
    pub fn push_expression(&mut self, value: GeneratorExpression) {
        self.expressions.push(value);
    }

    pub fn finish(self) -> GroupExpression {
        GroupExpression::new(self.expressions)
    }
}
