//!
//! The transpiler scope variable.
//!

use parser::TypeVariant;

#[derive(Debug)]
pub struct Variable {
    pub type_variant: TypeVariant,
    pub is_mutable: bool,
}

impl Variable {
    pub fn new(type_variant: TypeVariant, is_mutable: bool) -> Self {
        Self {
            type_variant,
            is_mutable,
        }
    }
}
