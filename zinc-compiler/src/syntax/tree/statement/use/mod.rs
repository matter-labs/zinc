//!
//! The use statement.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub path: ExpressionTree,
}

impl Statement {
    pub fn new(location: Location, path: ExpressionTree) -> Self {
        Self { location, path }
    }
}
