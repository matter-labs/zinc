//!
//! The expression object.
//!

use std::fmt;

use serde_derive::Serialize;

use super::Operand;
use super::Operator;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Object {
    Operator(Operator),
    Operand(Operand),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Operator(operator) => write!(f, "{}", operator),
            Object::Operand(operand) => write!(f, "{}", operand),
        }
    }
}
