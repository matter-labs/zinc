//!
//! The module statement.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Mod {
    pub location: Location,
    pub identifier: Identifier,
}

impl Mod {
    pub fn new(location: Location, identifier: Identifier) -> Self {
        Self {
            location,
            identifier,
        }
    }
}
