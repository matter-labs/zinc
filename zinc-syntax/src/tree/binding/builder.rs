//!
//! The binding builder.
//!

use zinc_lexical::Location;

use crate::tree::binding::Binding;
use crate::tree::pattern_binding::Pattern as BindingPattern;
use crate::tree::r#type::Type;

///
/// The binding pattern builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The binding.
    pattern: Option<BindingPattern>,
    /// The optional binding type.
    r#type: Option<Type>,
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
    pub fn set_pattern(&mut self, value: BindingPattern) {
        self.pattern = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> Binding {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let pattern = self.pattern.take().unwrap_or_else(|| {
            panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "binding")
        });

        Binding::new(location, pattern, self.r#type.take())
    }
}
