//!
//! The require statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::lexical::StringLiteral;
use crate::syntax::Expression;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Require {
    #[serde(skip_serializing)]
    pub location: Location,
    pub id: String,
    pub expression: Expression,
}

impl Require {
    pub fn new(location: Location, expression: Expression, tag: Option<StringLiteral>) -> Self {
        let id = tag
            .as_ref()
            .map(|tag| format!("{}", tag))
            .unwrap_or_else(|| format!("{}_{}", location.line(), location.column()));

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
