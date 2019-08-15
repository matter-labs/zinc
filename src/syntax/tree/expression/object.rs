//!
//! The expression object.
//!

use serde_derive::Serialize;

use super::Operand;
use super::Operator;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Object {
    Operator(Operator),
    Operand(Operand),
}
