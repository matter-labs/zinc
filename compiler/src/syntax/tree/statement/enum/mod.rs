//!
//! The struct statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Variant;

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub location: Location,
    pub identifier: Identifier,
    pub variants: Vec<Variant>,
}

impl Enum {
    pub fn new(location: Location, identifier: Identifier, variants: Vec<Variant>) -> Self {
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
                .map(|variant| variant.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
