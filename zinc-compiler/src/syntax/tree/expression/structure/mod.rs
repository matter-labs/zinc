//!
//! The structure expression.
//!

mod builder;

pub use self::builder::Builder;

use crate::lexical::Location;
use crate::syntax;
use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub path: syntax::Expression,
    pub fields: Vec<(Identifier, syntax::Expression)>,
}

impl Expression {
    pub fn new(
        location: Location,
        path: syntax::Expression,
        fields: Vec<(Identifier, syntax::Expression)>,
    ) -> Self {
        Self {
            location,
            path,
            fields,
        }
    }
}
