//!
//! The semantic analyzer scope declaration.
//!

use crate::syntax::TypeVariant;

#[derive(Debug, Clone)]
pub struct Declaration {
    pub type_variant: TypeVariant,
    pub is_mutable: bool,
}

impl Declaration {
    pub fn new(type_variant: TypeVariant, is_mutable: bool) -> Self {
        Self {
            type_variant,
            is_mutable,
        }
    }
}
