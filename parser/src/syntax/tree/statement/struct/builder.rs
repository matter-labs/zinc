//!
//! The struct statement builder.
//!

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::StructStatement;
use crate::syntax::Type;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    fields: Vec<(Identifier, Option<Type>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn push_field_identifier(&mut self, value: Identifier) {
        self.fields.push((value, None));
    }

    pub fn push_field_type(&mut self, value: Type) {
        self.fields.last_mut().expect("Missing field identifier").1 = Some(value);
    }

    pub fn finish(mut self) -> StructStatement {
        StructStatement::new(
            self.location.take().expect("Missing location"),
            self.identifier.take().expect("Missing identifier"),
            self.fields
                .into_iter()
                .map(|(identifier, r#type)| (identifier, r#type.expect("Missing field type")))
                .collect::<Vec<(Identifier, Type)>>(),
        )
    }
}
