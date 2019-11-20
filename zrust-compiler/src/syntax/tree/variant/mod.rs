//!
//! The variant.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub location: Location,
    pub identifier: Identifier,
    pub literal: IntegerLiteral,
}

impl Variant {
    pub fn new(location: Location, identifier: Identifier, literal: IntegerLiteral) -> Self {
        Self {
            location,
            identifier,
            literal,
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.identifier, self.literal)
    }
}
