//!
//! The require statement.
//!

mod builder;

pub use self::builder::Builder;

use std::fmt;

use serde_derive::Serialize;

use crate::syntax::Expression;

#[derive(Debug, Serialize, PartialEq)]
pub struct Require {
    expression: Expression,
}

impl Require {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }
}

impl fmt::Display for Require {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "require ( {} )", self.expression,)
    }
}
