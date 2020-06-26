//!
//! The tuple expression.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;

///
/// The tuple expression.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// The location of the syntax construction.
    pub location: Location,
    /// The tuple expression inner element expressions.
    pub elements: Vec<ExpressionTree>,
}

impl Expression {
    ///
    /// Creates a tuple expression.
    ///
    pub fn new(location: Location, elements: Vec<ExpressionTree>) -> Self {
        Self { location, elements }
    }
}
