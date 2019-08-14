//!
//! The require statement builder.
//!

use crate::syntax::Expression;
use crate::syntax::Require;

#[derive(Default)]
pub struct Builder {
    expression: Option<Expression>,
}

impl Builder {
    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> Require {
        Require::new(self.expression.take().expect("Missing expression"))
    }
}
