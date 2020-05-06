//!
//! The semantic analyzer unit value element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

///
/// Simple wrapper around the unit value.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub location: Option<Location>,
}

impl Unit {
    pub fn new(location: Option<Location>) -> Self {
        Self { location }
    }

    pub fn r#type(&self) -> Type {
        Type::unit(self.location)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<unit> value")
    }
}
