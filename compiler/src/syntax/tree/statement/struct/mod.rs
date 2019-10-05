//!
//! The struct statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub location: Location,
    pub identifier: Identifier,
    pub fields: Vec<(Identifier, Type)>,
}

impl Struct {
    pub fn new(
        location: Location,
        identifier: Identifier,
        fields: Vec<(Identifier, Type)>,
    ) -> Self {
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
                .map(|(identifier, r#type)| format!("{}: {}", identifier, r#type))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
