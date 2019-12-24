//!
//! The binding pattern builder.
//!

use crate::lexical::Location;
use crate::syntax::BindingPattern;
use crate::syntax::BindingPatternVariant;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    binding: Option<Identifier>,
    is_mutable: bool,
    ignoring: bool,
    r#type: Option<Type>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_binding(&mut self, value: Identifier) {
        self.binding = Some(value);
    }

    pub fn set_mutable(&mut self) {
        self.is_mutable = true;
    }

    pub fn set_ignoring(&mut self) {
        self.ignoring = true;
    }

    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    pub fn finish(mut self) -> BindingPattern {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let variant = if self.ignoring {
            BindingPatternVariant::Ignoring
        } else if let Some(identifier) = self.binding.take() {
            if self.is_mutable {
                BindingPatternVariant::MutableBinding(identifier)
            } else {
                BindingPatternVariant::Binding(identifier)
            }
        } else {
            panic!(
                "{}{}",
                crate::syntax::PANIC_BUILDER_REQUIRES_VALUE,
                "binding | ignoring"
            );
        };

        let r#type = self
            .r#type
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::syntax::PANIC_BUILDER_REQUIRES_VALUE, "type"));

        BindingPattern::new(location, variant, r#type)
    }
}
