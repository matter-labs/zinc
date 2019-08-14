//!
//! The require statement.
//!

mod builder;

pub use self::builder::Builder;

use serde_derive::Serialize;

use crate::lexical::Token;

#[derive(Debug, Serialize, PartialEq)]
pub struct Require {
    expression: Vec<Token>,
}

impl Require {
    pub fn new(expression: Vec<Token>) -> Self {
        Self { expression }
    }
}
