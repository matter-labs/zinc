//!
//! The expression element.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Token;

use super::object::Object;

#[derive(Debug, Serialize, PartialEq)]
pub struct Element {
    #[serde(flatten)]
    object: Object,
    #[serde(skip_serializing)]
    token: Token,
}

impl Element {
    pub fn new(object: Object, token: Token) -> Self {
        Self { object, token }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.object)
    }
}
