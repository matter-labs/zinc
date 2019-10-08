//!
//! The interpreter scope variable.
//!

use crate::Value;

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
