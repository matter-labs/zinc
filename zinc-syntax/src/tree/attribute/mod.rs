//!
//! The attribute.
//!

pub mod builder;
pub mod element;

use zinc_lexical::Location;

use self::element::Element;

///
/// The attribute.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    /// The location of the syntax construction.
    pub location: Location,
    /// If the attribute is related to the enclosing item, e.g. a module or block.
    pub is_inner: bool,
    /// The attribute elements.
    pub elements: Vec<Element>,
}

impl Attribute {
    ///
    /// Creates the attribute value.
    ///
    pub fn new(location: Location, is_inner: bool, elements: Vec<Element>) -> Self {
        Self {
            location,
            is_inner,
            elements,
        }
    }
}
