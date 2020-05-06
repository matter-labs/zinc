//!
//! The semantic analyzer boolean value element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

///
/// Simple wrapper around the boolean value.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub location: Option<Location>,
}

impl Boolean {
    pub fn new(location: Option<Location>) -> Self {
        Self { location }
    }

    pub fn r#type(&self) -> Type {
        Type::boolean(self.location)
    }

    pub fn has_the_same_type_as(&self, other: &Self) -> bool {
        self.r#type() == other.r#type()
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<boolean> value")
    }
}
