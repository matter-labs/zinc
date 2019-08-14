//!
//! The debug statement.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Token;

#[derive(Debug, Serialize, PartialEq)]
pub struct Debug {
    expression: Vec<Token>,
}

impl Debug {
    pub fn new(expression: Vec<Token>) -> Self {
        Self { expression }
    }
}
