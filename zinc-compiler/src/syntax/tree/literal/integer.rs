//!
//! The integer literal.
//!

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub location: Location,
    pub data: IntegerLiteral,
}

impl Literal {
    pub fn new(location: Location, data: IntegerLiteral) -> Self {
        Self { location, data }
    }
}

impl Into<usize> for Literal {
    fn into(self) -> usize {
        self.data.into()
    }
}
