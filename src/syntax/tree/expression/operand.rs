//!
//! The expression operand.
//!

use serde_derive::Serialize;

use crate::lexical::Identifier;
use crate::lexical::Literal;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Operand {
    Literal(Literal),
    Identifier(Identifier),
}
