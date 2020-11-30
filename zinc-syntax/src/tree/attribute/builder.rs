//!
//! The attribute builder.
//!

use zinc_lexical::Location;

use crate::tree::attribute::element::Element as AttributeElement;
use crate::tree::attribute::Attribute;

///
/// The attribute builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// If the attribute is related to the enclosing item, e.g. a module or block.
    is_inner: bool,
    /// The attribute element.
    elements: Vec<AttributeElement>,
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
    pub fn set_elements(&mut self, value: Vec<AttributeElement>) {
        self.elements = value;
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

        Attribute::new(location, self.is_inner, self.elements)
    }
}
