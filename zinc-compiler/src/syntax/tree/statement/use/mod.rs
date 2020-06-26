//!
//! The `use` statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

///
/// The `use` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The imported item path expression.
    pub path: ExpressionTree,
}

impl Statement {
    ///
    /// Creates a `use` statement.
    ///
    pub fn new(location: Location, path: ExpressionTree) -> Self {
        Self { location, path }
    }
}
