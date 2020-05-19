//!
//! The semantic analyzer argument list element.
//!

use std::fmt;

use crate::lexical::token::location::Location;
use crate::semantic::element::Element;

///
/// A function argument list.
///
#[derive(Debug, Clone)]
pub struct ArgumentList {
    pub location: Location,
    pub arguments: Vec<Element>,
}

impl ArgumentList {
    pub fn new(location: Location, arguments: Vec<Element>) -> Self {
        Self {
            location,
            arguments,
        }
    }
}

impl fmt::Display for ArgumentList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.arguments
                .iter()
                .map(|element| element.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
