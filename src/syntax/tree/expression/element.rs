//!
//! The expression element.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Token;
use crate::syntax::ExpressionObject;

#[derive(Debug, Serialize, PartialEq)]
pub struct Element {
    #[serde(flatten)]
    pub object: ExpressionObject,
    #[serde(skip_serializing)]
    pub token: Token,
}

impl Element {
    pub fn new(object: ExpressionObject, token: Token) -> Self {
        Self { object, token }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.object)
    }
}
