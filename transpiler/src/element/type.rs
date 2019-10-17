//!
//! Transpiler type element.
//!

use std::fmt;

use parser::TypeVariant;

#[derive(Debug, Clone)]
pub struct Element {
    pub type_variant: TypeVariant,
}

impl Element {
    pub fn new(type_variant: TypeVariant) -> Self {
        Self { type_variant }
    }
}

impl Into<String> for Element {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_variant)
    }
}
