//!
//! The semantic analyzer tuple type element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::Type;

#[derive(Debug, Clone)]
pub struct Tuple {
    pub location: Option<Location>,
    pub types: Vec<Type>,
}

impl Tuple {
    pub fn new(location: Option<Location>, types: Vec<Type>) -> Self {
        Self { location, types }
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.types
                .iter()
                .map(|r#type| r#type.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
