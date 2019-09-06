//!
//! The identifier.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Identifier {
    #[serde(skip_serializing)]
    location: Location,
    name: String,
}

impl Identifier {
    pub fn new(location: Location, name: String) -> Self {
        Self { location, name }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
