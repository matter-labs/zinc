//!
//! The match expression.
//!

pub mod builder;

use crate::lexical::Location;
use crate::syntax::tree::expression::Expression as SyntaxExpression;
use crate::syntax::tree::pattern_match::Pattern as MatchPattern;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub scrutinee: SyntaxExpression,
    pub branches: Vec<(MatchPattern, SyntaxExpression)>,
}

impl Expression {
    pub fn new(
        location: Location,
        scrutinee: SyntaxExpression,
        branches: Vec<(MatchPattern, SyntaxExpression)>,
    ) -> Self {
        Self {
            location,
            scrutinee,
            branches,
        }
    }
}
