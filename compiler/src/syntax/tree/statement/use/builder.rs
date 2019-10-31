//!
//! The use statement builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::UseStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    path: Option<Expression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_path(&mut self, value: Expression) {
        self.path = Some(value);
    }

    pub fn finish(mut self) -> UseStatement {
        UseStatement::new(
            self.location.take().expect("Missing location"),
            self.path.take().expect("Missing path"),
        )
    }
}
