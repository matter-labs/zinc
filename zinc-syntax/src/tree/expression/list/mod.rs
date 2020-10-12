//!
//! The function argument list expression.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The function argument list expression.
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Expression {
    /// The location of the syntax construction.
    pub location: Location,
    /// The function argument list inner expressions.
    pub elements: Vec<ExpressionTree>,
}

impl Expression {
    ///
    /// Creates an argument list expression.
    ///
    pub fn new(location: Location, elements: Vec<ExpressionTree>) -> Self {
        Self { location, elements }
    }

    ///
    /// The argument list length.
    ///
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    ///
    /// Whether the argument list is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
