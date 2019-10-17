//!
//! Transpiler temporary element.
//!

use std::fmt;

use parser::TypeVariant;

#[derive(Debug)]
pub struct Element {
    pub identifier: String,
    pub type_variant: TypeVariant,
}

impl Element {
    pub fn new(identifier: String, type_variant: TypeVariant) -> Self {
        Self {
            identifier,
            type_variant,
        }
    }
}

impl Into<String> for Element {
    fn into(self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.identifier)
    }
}
