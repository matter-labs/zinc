//!
//! The semantic analyzer scope assignment.
//!

use crate::syntax::TypeVariant;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub type_variant: TypeVariant,
    pub address: usize,
    pub is_outer: bool,
}

impl Assignment {
    pub fn new(type_variant: TypeVariant, address: usize, is_outer: bool) -> Self {
        Self {
            type_variant,
            address,
            is_outer,
        }
    }
}
