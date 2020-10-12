//!
//! The semantic analyzer constant string element.
//!

use std::fmt;

use zinc_lexical::Location;
use zinc_syntax::StringLiteral;

use crate::semantic::element::r#type::i_typed::ITyped;
use crate::semantic::element::r#type::Type;

///
/// Simple wrapper around the `std::string::String` value.
///
#[derive(Debug, Clone, PartialEq)]
pub struct String {
    /// The location, where the value appears in the code.
    pub location: Location,
    /// The inner string value.
    pub inner: ::std::string::String,
}

impl String {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, inner: ::std::string::String) -> Self {
        Self { location, inner }
    }
}

impl ITyped for String {
    fn r#type(&self) -> Type {
        Type::string(Some(self.location))
    }

    fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl From<StringLiteral> for String {
    fn from(literal: StringLiteral) -> Self {
        Self {
            location: literal.location,
            inner: literal.into(),
        }
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.inner)
    }
}
