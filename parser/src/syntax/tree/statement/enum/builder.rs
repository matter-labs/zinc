//!
//! The struct statement builder.
//!

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;
use crate::syntax::EnumStatement;
use crate::syntax::Identifier;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    variants: Vec<(Identifier, Option<IntegerLiteral>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn push_variant_identifier(&mut self, value: Identifier) {
        self.variants.push((value, None));
    }

    pub fn push_variant_value(&mut self, value: IntegerLiteral) {
        self.variants
            .last_mut()
            .expect("Missing variant identifier")
            .1 = Some(value);
    }

    pub fn finish(mut self) -> EnumStatement {
        EnumStatement::new(
            self.location.take().expect("Missing location"),
            self.identifier.take().expect("Missing identifier"),
            self.variants
                .into_iter()
                .map(|(identifier, value)| (identifier, value.expect("Missing variant value")))
                .collect::<Vec<(Identifier, IntegerLiteral)>>(),
        )
    }
}
