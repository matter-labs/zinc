//!
//! The expression element.
//!

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
