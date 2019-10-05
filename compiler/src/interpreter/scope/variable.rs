//!
//! The interpreter scope variable.
//!

use crate::interpreter::Value;

#[derive(Debug)]
pub struct Variable {
    pub value: Value,
    pub is_mutable: bool,
}

impl Variable {
    pub fn new(value: Value, is_mutable: bool) -> Self {
        Self { value, is_mutable }
    }
}
