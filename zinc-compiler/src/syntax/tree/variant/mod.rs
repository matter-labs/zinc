//!
//! The variant.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

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
