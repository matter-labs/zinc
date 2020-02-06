//!
//! The structure or identifier expression.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub identifier: Identifier,
    pub is_struct: bool,
    pub fields: Vec<(Identifier, syntax::Expression)>,
}

impl Expression {
    pub fn new(
        location: Location,
        identifier: Identifier,
        is_struct: bool,
        fields: Vec<(Identifier, syntax::Expression)>,
    ) -> Self {
        Self {
            location,
            identifier,
            is_struct,
            fields,
        }
    }
}
