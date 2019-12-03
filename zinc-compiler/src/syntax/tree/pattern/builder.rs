//!
//! The pattern builder.
//!

use crate::lexical::Location;
use crate::syntax::BooleanLiteral;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::Pattern;
use crate::syntax::PatternVariant;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    boolean_literal: Option<BooleanLiteral>,
    integer_literal: Option<IntegerLiteral>,
    binding: Option<Identifier>,
    ignoring: bool,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_boolean_literal(&mut self, value: BooleanLiteral) {
        self.boolean_literal = Some(value);
    }

    pub fn set_integer_literal(&mut self, value: IntegerLiteral) {
        self.integer_literal = Some(value);
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
        } else if let Some(boolean_literal) = self.boolean_literal.take() {
            PatternVariant::BooleanLiteral(boolean_literal)
        } else if let Some(integer_literal) = self.integer_literal.take() {
            PatternVariant::IntegerLiteral(integer_literal)
        } else if let Some(identifier) = self.binding.take() {
            PatternVariant::Binding(identifier)
        } else {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "boolean | integer | binding | ignoring"
            );
        };

        Pattern::new(location, variant)
    }
}
