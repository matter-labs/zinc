//!
//! The impl statement.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::ImplementationLocalStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    pub location: Location,
    pub identifier: Identifier,
    pub statements: Vec<ImplementationLocalStatement>,
}

impl Impl {
    pub fn new(
        location: Location,
        identifier: Identifier,
        statements: Vec<ImplementationLocalStatement>,
    ) -> Self {
        Self {
            location,
            identifier,
            statements,
        }
    }
}
