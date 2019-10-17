//!
//! The struct statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::IntegerLiteral;
use crate::lexical::Location;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub location: Location,
    pub identifier: Identifier,
    pub variants: Vec<(Identifier, IntegerLiteral)>,
}

impl Enum {
    pub fn new(
        location: Location,
        identifier: Identifier,
        variants: Vec<(Identifier, IntegerLiteral)>,
    ) -> Self {
        Self {
            location,
            identifier,
            variants,
        }
    }
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "enum {} {{ {} }}",
            self.identifier,
            self.variants
                .iter()
                .map(|(identifier, value)| format!("{}: {}", identifier, value))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
