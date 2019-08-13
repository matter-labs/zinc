//!
//! The require expression.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Token;

#[derive(Debug, Serialize, PartialEq)]
pub struct Let {
    expression: Vec<Token>,
}

impl Let {
    pub fn new(expression: Vec<Token>) -> Self {
        Self { expression }
    }
}
