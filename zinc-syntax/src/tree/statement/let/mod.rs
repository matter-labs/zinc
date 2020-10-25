//!
//! The `let` statement.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::binding::Binding;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The `let` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The binding pattern.
    pub binding: Binding,
    /// The expression assigned to the variable.
    pub expression: ExpressionTree,
}

impl Statement {
    ///
    /// Creates a `let` statement.
    ///
    pub fn new(location: Location, binding: Binding, expression: ExpressionTree) -> Self {
        Self {
            location,
            binding,
            expression,
        }
    }
}
