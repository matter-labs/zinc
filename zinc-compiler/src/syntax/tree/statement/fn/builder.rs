//!
//! The fn statement builder.
//!

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::Field;
use crate::syntax::FnStatement;
use crate::syntax::Identifier;
use crate::syntax::Type;
use crate::syntax::TypeVariant;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    arguments: Vec<Field>,
    return_type: Option<Type>,
    body: Option<BlockExpression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_arguments(&mut self, value: Vec<Field>) {
        self.arguments = value;
    }

    pub fn set_return_type(&mut self, value: Type) {
        self.return_type = Some(value);
    }

    pub fn set_body(&mut self, value: BlockExpression) {
        self.body = Some(value);
    }

    pub fn finish(mut self) -> FnStatement {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "location"
            )
        });
        FnStatement::new(
            location,
            self.identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                    "identifier"
                )
            }),
            self.arguments,
            self.return_type
                .unwrap_or_else(|| Type::new(location, TypeVariant::new_unit())),
            self.body.take().unwrap_or_else(|| {
                panic!("{}{}", crate::syntax::PANIC_BUILDER_REQUIRES_VALUE, "body")
            }),
        )
    }
}
