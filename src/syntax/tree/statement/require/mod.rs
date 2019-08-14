//!
//! The require statement.
//!

mod builder;

pub use self::builder::Builder;

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
