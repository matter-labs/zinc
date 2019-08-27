//!
//! The require statement builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::Require;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    expression: Option<Expression>,
    tag: Option<String>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn set_tag(&mut self, value: String) {
        self.tag = Some(value);
    }

    pub fn finish(mut self) -> Require {
        Require::new(
            self.location.take().expect("Missing location"),
            self.expression.take().expect("Missing expression"),
            self.tag.take(),
        )
    }
}
