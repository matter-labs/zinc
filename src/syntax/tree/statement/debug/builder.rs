//!
//! The debug statement builder.
//!

use crate::lexical::Token;
use crate::syntax::Debug;

#[derive(Default)]
pub struct Builder {
    expression: Option<Vec<Token>>,
}

impl Builder {
    pub fn set_expression(&mut self, value: Vec<Token>) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> Debug {
        Debug::new(self.expression.take().expect("Missing expression"))
    }
}
