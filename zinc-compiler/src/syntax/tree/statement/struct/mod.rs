//!
//! The struct statement.
//!

mod builder;

pub use self::builder::Builder;

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
