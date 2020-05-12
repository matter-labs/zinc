//!
//! The semantic analyzer constant unit element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

///
/// Simple wrapper around the constant unit value.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub location: Location,
}

impl Unit {
    pub fn new(location: Location) -> Self {
        Self { location }
    }

    pub fn r#type(&self) -> Type {
        Type::unit(None)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'()'")
    }
}
