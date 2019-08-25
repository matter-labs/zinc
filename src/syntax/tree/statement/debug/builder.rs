//!
//! The debug statement builder.
//!

use crate::lexical::Location;
use crate::syntax::Debug;
use crate::syntax::Expression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    expression: Option<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn finish(mut self) -> Debug {
        Debug::new(
            self.location.take().expect("Missing location"),
            self.expression.take().expect("Missing expression"),
        )
    }
}
