//!
//! The semantic analyzer constant string element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;
use crate::syntax::tree::literal::string::Literal as StringLiteral;

///
/// Simple wrapper around the `std::string::String` value.
///
#[derive(Debug, Clone, PartialEq)]
pub struct String {
    pub location: Location,
    pub inner: std::string::String,
}

impl String {
    pub fn new(location: Location, inner: std::string::String) -> Self {
        Self { location, inner }
    }

    pub fn r#type(&self) -> Type {
        Type::string(Some(self.location))
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
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
