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
}

impl Require {
    pub fn new(location: Location, expression: Expression, tag: Option<String>) -> Self {
        Self {
            location,
            expression,
            tag,
        }
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "require {} ( {} )",
            self.tag
                .as_ref()
                .map(|tag| format!("\"{}\"", tag))
                .unwrap_or_else(|| self.location.to_string()),
            self.expression,
        )
    }
}
