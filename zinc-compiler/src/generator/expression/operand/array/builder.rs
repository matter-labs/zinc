//!
//! The generator expression array operand builder.
//!

use crate::generator::expression::operand::array::Expression as ArrayExpression;
use crate::generator::expression::Expression as GeneratorExpression;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    expressions: Vec<GeneratorExpression>,
    size: Option<usize>,
}

impl Builder {
    pub fn push_expression(&mut self, value: GeneratorExpression) {
        self.expressions.push(value);
    }

    pub fn set_size(&mut self, value: usize) {
        self.size = Some(value);
    }

    pub fn finish(mut self) -> ArrayExpression {
        match self.size.take() {
            Some(size) => {
                let expression = self.expressions.pop().unwrap_or_else(|| {
                    panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "expression")
                });

                ArrayExpression::new_repeated(expression, size)
            }
            None => ArrayExpression::new_list(self.expressions),
        }
    }
}
