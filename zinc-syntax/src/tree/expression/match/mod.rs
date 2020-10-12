//!
//! The match expression.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::pattern_match::Pattern as MatchPattern;

///
/// The match expression.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// The location of the syntax construction.
    pub location: Location,
    /// The match scrutinee expression, which is the matched expression.
    pub scrutinee: ExpressionTree,
    /// The match pattern-expression pairs.
    pub branches: Vec<(MatchPattern, ExpressionTree)>,
}

impl Expression {
    ///
    /// Creates a match expression.
    ///
    pub fn new(
        location: Location,
        scrutinee: ExpressionTree,
        branches: Vec<(MatchPattern, ExpressionTree)>,
    ) -> Self {
        Self {
            location,
            scrutinee,
            branches,
        }
    }
}
