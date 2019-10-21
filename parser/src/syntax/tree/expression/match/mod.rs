//!
//! The match expression.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::syntax;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub location: Location,
    pub match_expression: syntax::Expression,
    pub branches: Vec<(syntax::Expression, syntax::Expression)>,
}

impl Expression {
    pub fn new(
        location: Location,
        match_expression: syntax::Expression,
        branches: Vec<(syntax::Expression, syntax::Expression)>,
    ) -> Self {
        Self {
            location,
            match_expression,
            branches,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "match {} {{ {} }}",
            self.match_expression,
            self.branches
                .iter()
                .map(|(left, right)| format!("{} => {}", left, right))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
