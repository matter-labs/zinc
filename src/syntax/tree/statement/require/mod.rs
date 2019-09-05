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
    pub location: Location,
    pub id: String,
    pub expression: Expression,
    pub tag: Option<StringLiteral>,
}

impl Require {
    pub fn new(location: Location, expression: Expression, tag: Option<StringLiteral>) -> Self {
        let id = tag
            .as_ref()
            .map(|tag| format!("\"{}\"", tag))
            .unwrap_or_else(|| location.to_string());

        Self {
            location,
            id,
            expression,
            tag,
        }
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "require ({}, {})", self.expression, self.id)
    }
}
