//!
//! The enum statement.
//!

mod builder;

pub use self::builder::Builder;

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
