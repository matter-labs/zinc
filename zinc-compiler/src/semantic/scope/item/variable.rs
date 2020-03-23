//!
//! The semantic analyzer scope variable item.
//!

use crate::semantic::element::r#type::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub is_mutable: bool,
    pub r#type: Type,
}

impl Variable {
    pub fn new(is_mutable: bool, r#type: Type) -> Self {
        Self { is_mutable, r#type }
    }
}
