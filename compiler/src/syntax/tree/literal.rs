//!
//! The literal.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical;
use crate::lexical::Location;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Literal {
    #[serde(skip_serializing)]
    location: Location,
    data: lexical::Literal,
}

impl Literal {
    pub fn new(location: Location, data: lexical::Literal) -> Self {
        Self { location, data }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn into_inner(self) -> lexical::Literal {
        self.data
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
