//!
//! The match expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::pattern_match::Pattern as MatchPattern;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub scrutinee: ExpressionTree,
    pub branches: Vec<(MatchPattern, ExpressionTree)>,
}

impl Expression {
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
