//!
//! The attribute builder.
//!

use zinc_lexical::Location;

use crate::tree::attribute::Attribute;
use crate::tree::identifier::Identifier;

///
/// The attribute builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// If the attribute is related to the enclosing item, e.g. a module or block.
    is_inner: bool,
    /// The attribute identifier.
    identifier: Option<Identifier>,
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
    pub fn set_inner(&mut self) {
        self.is_inner = true;
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> Attribute {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let identifier = self.identifier.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "identifier"
            )
        });

        Attribute::new(location, self.is_inner, identifier)
    }
}
