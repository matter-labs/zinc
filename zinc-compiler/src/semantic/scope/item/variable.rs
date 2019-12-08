//!
//! The semantic analyzer scope variable item.
//!

use crate::semantic::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub r#type: Type,
    pub is_mutable: bool,
    pub address: usize,
}

impl Variable {
    pub fn new(r#type: Type, is_mutable: bool, address: usize) -> Self {
        Self {
            r#type,
            is_mutable,
            address,
        }
    }
}
