//!
//! The use statement.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub path: Expression,
}

impl Statement {
    pub fn new(location: Location, path: Expression) -> Self {
        Self { location, path }
    }
}
