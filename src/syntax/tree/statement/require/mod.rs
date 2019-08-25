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
}

impl Require {
    pub fn new(location: Location, expression: Expression) -> Self {
        Self {
            location,
            expression,
        }
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "require ( {} )", self.expression)
    }
}
