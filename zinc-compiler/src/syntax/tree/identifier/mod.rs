//!
//! The identifier.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub location: Location,
    pub name: String,
}

impl Identifier {
    pub fn new(location: Location, name: String) -> Self {
        Self { location, name }
    }
}
