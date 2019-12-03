//!
//! The literal.
//!

use crate::lexical::BooleanLiteral;
use crate::lexical::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub location: Location,
    pub data: BooleanLiteral,
}

impl Literal {
    pub fn new(location: Location, data: BooleanLiteral) -> Self {
        Self { location, data }
    }
}

impl Into<bool> for Literal {
    fn into(self) -> bool {
        self.data.into()
    }
}
