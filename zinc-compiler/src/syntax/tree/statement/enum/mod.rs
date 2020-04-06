//!
//! The enum statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::variant::Variant;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub variants: Vec<Variant>,
}

impl Statement {
    pub fn new(location: Location, identifier: Identifier, variants: Vec<Variant>) -> Self {
        Self {
            location,
            identifier,
            variants,
        }
    }
}
