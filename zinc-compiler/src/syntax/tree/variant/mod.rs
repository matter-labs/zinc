//!
//! The variant.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;

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
