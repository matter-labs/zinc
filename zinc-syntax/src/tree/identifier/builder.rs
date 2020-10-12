//!
//! The identifier builder.
//!

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;

///
/// The identifier builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The identifier string contents.
    name: Option<String>,
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
    pub fn set_name(&mut self, value: String) {
        self.name = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> Identifier {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let name = self
            .name
            .take()
            .unwrap_or_else(|| panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "name"));

        Identifier::new(location, name)
    }
}
