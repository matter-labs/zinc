//!
//! The semantic analyzer boolean value element.
//!

use std::fmt;

use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;
use zinc_lexical::Location;

///
/// Simple wrapper around the `bool` value.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    /// The location, where the value appears in the code.
    pub location: Option<Location>,
}

impl Boolean {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Option<Location>) -> Self {
        Self { location }
    }
}

impl ITyped for Boolean {
    fn r#type(&self) -> Type {
        Type::boolean(self.location)
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<runtime>")
    }
}
