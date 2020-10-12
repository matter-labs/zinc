//!
//! The `field` statement builder.
//!

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::r#type::Type;
use crate::tree::statement::field::Statement as FieldStatement;

///
/// The `field` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// If the contract storage field is public.
    is_public: bool,
    /// The contract storage field identifier.
    identifier: Option<Identifier>,
    /// The contract storage field type.
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
    pub fn set_public(&mut self) {
        self.is_public = true;
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
    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> FieldStatement {
        FieldStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.is_public,
            self.identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "identifier"
                )
            }),
            self.r#type.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "type")
            }),
        )
    }
}
