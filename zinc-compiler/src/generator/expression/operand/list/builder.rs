//!
//! The generator expression tuple operand builder.
//!

use crate::generator::expression::operand::list::Expression as ListExpression;
use crate::generator::expression::Expression as GeneratorExpression;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    expressions: Vec<GeneratorExpression>,
}

impl Builder {
    pub fn push_expression(&mut self, value: GeneratorExpression) {
        self.expressions.push(value);
    }

    pub fn finish(self) -> ListExpression {
        ListExpression::new(self.expressions)
    }
}
