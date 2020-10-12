//!
//! The `enum` statement builder.
//!

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::statement::r#enum::Statement as EnumStatement;
use crate::tree::variant::Variant;

///
/// The `enum` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The enumeration type identifier.
    identifier: Option<Identifier>,
    /// The enumeration type variants.
    variants: Vec<Variant>,
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
    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_variants(&mut self, value: Vec<Variant>) {
        self.variants = value;
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> EnumStatement {
        EnumStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "identifier"
                )
            }),
            self.variants,
        )
    }
}
