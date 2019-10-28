//!
//! The type statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub location: Location,
    pub identifier: Identifier,
    pub r#type: syntax::Type,
}

impl Type {
    pub fn new(location: Location, identifier: Identifier, r#type: syntax::Type) -> Self {
        Self {
            location,
            identifier,
            r#type,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type {} = {}", self.identifier, self.r#type)
    }
}
