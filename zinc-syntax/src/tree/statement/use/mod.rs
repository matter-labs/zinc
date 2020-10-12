//!
//! The `use` statement.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;

///
/// The `use` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The imported item path expression.
    pub path: ExpressionTree,
    /// The imported item optional alias.
    pub alias_identifier: Option<Identifier>,
}

impl Statement {
    ///
    /// Creates a `use` statement.
    ///
    pub fn new(
        location: Location,
        path: ExpressionTree,
        alias_identifier: Option<Identifier>,
    ) -> Self {
        Self {
            location,
            path,
            alias_identifier,
        }
    }
}
