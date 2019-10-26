//!
//! The struct statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax::Field;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub location: Location,
    pub identifier: Identifier,
    pub fields: Vec<Field>,
}

impl Struct {
    pub fn new(location: Location, identifier: Identifier, fields: Vec<Field>) -> Self {
        Self {
            location,
            identifier,
            fields,
        }
    }
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "struct {} {{ {} }}",
            self.identifier,
            self.fields
                .iter()
                .map(|field| field.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
