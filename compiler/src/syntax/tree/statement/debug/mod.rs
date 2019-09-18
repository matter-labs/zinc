//!
//! The debug statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;
use crate::syntax::Expression;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Debug {
    pub location: Location,
    pub expression: Expression,
}

impl Debug {
    pub fn new(location: Location, expression: Expression) -> Self {
        Self {
            location,
            expression,
        }
    }
}

impl fmt::Display for Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debug ({})", self.expression,)
    }
}
