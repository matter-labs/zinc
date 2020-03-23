//!
//! The use statement builder.
//!

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression;
use crate::syntax::tree::statement::r#use::Statement as UseStatement;

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
            self.location
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location")),
            self.path
                .take()
                .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "path")),
        )
    }
}
