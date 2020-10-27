//!
//! The binding pattern builder.
//!

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::pattern_binding::variant::Variant as BindingPatternVariant;
use crate::tree::pattern_binding::Pattern as BindingPattern;

///
/// The binding pattern builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// If the binding pattern is mutable.
    is_mutable: bool,
    /// The binding pattern identifier.
    identifier: Option<Identifier>,
    /// If the binding pattern is a wildcard.
    is_wildcard: bool,
    /// If the binding pattern is a tuple-like list.
    bindings: Vec<BindingPattern>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_mutable(&mut self) {
        self.is_mutable = true;
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_wildcard(&mut self) {
        self.is_wildcard = true;
    }

    ///
    /// Pushes a binding to the tuple-like binding list.
    ///
    /// Is used for parenthesized binding lists.
    ///
    pub fn push_binding(&mut self, value: BindingPattern) {
        self.bindings.push(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> BindingPattern {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let variant = if let Some(identifier) = self.identifier.take() {
            BindingPatternVariant::new_binding(identifier, self.is_mutable)
        } else if self.is_wildcard || self.bindings.is_empty() {
            BindingPatternVariant::new_wildcard()
        } else {
            BindingPatternVariant::new_binding_list(self.bindings)
        };

        BindingPattern::new(location, variant)
    }
}
