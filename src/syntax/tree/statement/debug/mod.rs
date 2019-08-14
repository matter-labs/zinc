//!
//! The debug statement.
//!

mod builder;

pub use self::builder::Builder;

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
