//!
//! The attribute element builder.
//!

use zinc_lexical::Location;

use crate::tree::attribute::element::variant::Variant as AttributeElementVariant;
use crate::tree::attribute::element::Element as AttributeElement;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::literal::Literal;

///
/// The attribute builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The attribute identifier path.
    path: Option<ExpressionTree>,
    /// The attribute value literal.
    value: Option<Literal>,
    /// The nested attribute.
    nested: Option<Vec<AttributeElement>>,
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
    pub fn set_path(&mut self, value: ExpressionTree) {
        self.path = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_value(&mut self, value: Literal) {
        self.value = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_nested(&mut self, value: Vec<AttributeElement>) {
        self.nested = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> AttributeElement {
        let location = self.location.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "location"
            )
        });

        let identifier = self.path.take().unwrap_or_else(|| {
            panic!(
                "{}{}",
                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                "identifier"
            )
        });

        let variant = if let Some(value) = self.value.take() {
            Some(AttributeElementVariant::Value(value))
        } else if let Some(nested) = self.nested.take() {
            Some(AttributeElementVariant::Nested(nested))
        } else {
            None
        };

        AttributeElement::new(location, identifier, variant)
    }
}
