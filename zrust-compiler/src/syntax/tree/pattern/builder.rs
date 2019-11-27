//!
//! The pattern builder.
//!

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Literal;
use crate::syntax::Pattern;
use crate::syntax::PatternVariant;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    literal: Option<Literal>,
    binding: Option<Identifier>,
    ignoring: bool,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_literal(&mut self, value: Literal) {
        self.literal = Some(value);
    }

    pub fn set_binding(&mut self, value: Identifier) {
        self.binding = Some(value);
    }

    pub fn set_ignoring(&mut self) {
        self.ignoring = true;
    }

    pub fn finish(mut self) -> Pattern {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let variant = if self.ignoring {
            PatternVariant::Ignoring
        } else if let Some(literal) = self.literal.take() {
            PatternVariant::Literal(literal)
        } else if let Some(identifier) = self.binding.take() {
            PatternVariant::Binding(identifier)
        } else {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "literal | binding | ignoring"
            );
        };

        Pattern::new(location, variant)
    }
}
