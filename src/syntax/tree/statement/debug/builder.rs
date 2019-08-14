//!
//! The debug statement builder.
//!

use crate::syntax::Debug;
use crate::syntax::Expression;

#[derive(Default)]
pub struct Builder {
    expression: Option<Expression>,
}

impl Builder {
    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> Debug {
        Debug::new(self.expression.take().expect("Missing expression"))
    }
}
