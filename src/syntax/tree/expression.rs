//!
//! The expression.
//!

use serde_derive::Serialize;

use crate::lexical::Token;

#[derive(Debug, Default, Serialize, PartialEq)]
pub struct Expression(Vec<Token>);

impl Expression {
    pub fn push(&mut self, value: Token) {
        self.0.push(value)
    }

    pub fn append(&mut self, mut expression: Expression) {
        self.0.append(&mut expression.0)
    }
}
