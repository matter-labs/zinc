//!
//! The require statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax::Expression;

#[derive(Debug, Serialize, PartialEq)]
pub struct Require {
    pub location: Location,
    pub expression: Expression,
    pub tag: Option<String>,
    pub id: String,
}

impl Require {
    pub fn new(location: Location, expression: Expression, tag: Option<String>) -> Self {
        let id = tag
            .as_ref()
            .map(|tag| format!("\"{}\"", tag))
            .unwrap_or_else(|| location.to_string());

        Self {
            location,
            expression,
            tag,
            id,
        }
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "require {} ( {} )", self.id, self.expression,)
    }
}
