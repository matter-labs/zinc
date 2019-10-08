//!
//! The require statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use crate::lexical::Location;
use crate::lexical::StringLiteral;
use crate::syntax::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct Require {
    pub location: Location,
    pub id: String,
    pub expression: Expression,
}

impl Require {
    pub fn new(location: Location, expression: Expression, tag: Option<StringLiteral>) -> Self {
        let id = tag
            .map(|literal| literal.to_string())
            .unwrap_or_else(|| format!("L{}", location.line));

        Self {
            location,
            id,
            expression,
        }
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "require ({}, {})", self.expression, self.id)
    }
}
