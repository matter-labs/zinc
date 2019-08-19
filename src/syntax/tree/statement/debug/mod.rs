//!
//! The debug statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::Expression;

#[derive(Debug, Serialize, PartialEq)]
pub struct Debug {
    expression: Expression,
}

impl Debug {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

impl fmt::Display for Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debug ( {} )", self.expression,)
    }
}
