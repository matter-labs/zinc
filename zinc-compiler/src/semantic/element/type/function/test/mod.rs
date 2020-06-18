//!
//! The semantic analyzer test function element.
//!

mod tests;

pub mod error;

use std::fmt;

use crate::lexical::token::location::Location;

#[derive(Debug, Clone)]
pub struct Function {
    pub location: Location,
    pub identifier: String,
    pub type_id: usize,
}

impl Function {
    pub fn new(location: Location, identifier: String, type_id: usize) -> Self {
        Self {
            location,
            identifier,
            type_id,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}()", self.identifier,)
    }
}
