//!
//! The semantic analyzer array type element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

#[derive(Debug, Clone)]
pub struct Array {
    pub location: Option<Location>,
    pub r#type: Box<Type>,
    pub size: usize,
}

impl Array {
    pub fn new(location: Option<Location>, r#type: Box<Type>, size: usize) -> Self {
        Self {
            location,
            r#type,
            size,
        }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}; {}]", self.r#type, self.size)
    }
}
