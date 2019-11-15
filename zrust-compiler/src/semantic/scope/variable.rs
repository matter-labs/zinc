//!
//! The semantic analyzer scope variable.
//!

use crate::semantic::Value;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Value,
    pub address: usize,
    pub is_mutable: bool,
}

impl Variable {
    pub fn new(value: Value, address: usize, is_mutable: bool) -> Self {
        Self {
            value,
            address,
            is_mutable,
        }
    }
}
