//!
//! The string literal.
//!

use crate::lexical::Location;
use crate::lexical::StringLiteral;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub location: Location,
    pub data: StringLiteral,
}

impl Literal {
    pub fn new(location: Location, data: StringLiteral) -> Self {
        Self { location, data }
    }
}
