//!
//! The let statement builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::Identifier;
use crate::syntax::Let;
use crate::syntax::Type;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    r#type: Option<Type>,
    expression: Option<Expression>,
    is_mutable: bool,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    pub fn set_expression(&mut self, value: Expression) {
        self.expression = Some(value);
    }

    pub fn set_mutable(&mut self) {
        self.is_mutable = true;
    }

    pub fn finish(mut self) -> Let {
        Let::new(
            self.location.take().expect("Missing location"),
            self.identifier.take().expect("Missing identifier"),
            self.is_mutable,
            self.r#type.take(),
            self.expression.take().expect("Missing expression"),
        )
    }
}
