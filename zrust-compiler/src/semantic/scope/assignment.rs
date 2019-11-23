//!
//! The semantic analyzer scope assignment.
//!

use crate::semantic::Value;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub value: Value,
    pub address: usize,
    pub is_outer: bool,
}

impl Assignment {
    pub fn new(value: Value, address: usize, is_outer: bool) -> Self {
        Self {
            value,
            address,
            is_outer,
        }
    }
}
