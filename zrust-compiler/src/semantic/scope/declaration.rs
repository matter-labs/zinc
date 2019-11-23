//!
//! The semantic analyzer scope declaration.
//!

use crate::semantic::Value;

#[derive(Debug, Clone)]
pub struct Declaration {
    pub value: Value,
    pub is_mutable: bool,
}

impl Declaration {
    pub fn new(value: Value, is_mutable: bool) -> Self {
        Self { value, is_mutable }
    }
}
