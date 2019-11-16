//!
//! The semantic analyzer scope variable.
//!

use crate::semantic::Value;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Value,
    pub is_mutable: bool,
}

impl Variable {
    pub fn new(value: Value, is_mutable: bool) -> Self {
        Self { value, is_mutable }
    }
}
