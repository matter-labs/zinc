//!
//! The literal.
//!

use std::fmt;

use crate::lexical;
use crate::lexical::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub location: Location,
    pub data: lexical::Literal,
}

impl Literal {
    pub fn new(location: Location, data: lexical::Literal) -> Self {
        Self { location, data }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
