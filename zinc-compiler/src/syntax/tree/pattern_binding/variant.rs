//!
//! The binding pattern variant.
//!

use crate::syntax::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Binding(Identifier),
    MutableBinding(Identifier),
    Ignoring,
}

impl Variant {
    pub fn new_binding(identifier: Identifier) -> Self {
        Self::Binding(identifier)
    }

    pub fn new_mutable_binding(identifier: Identifier) -> Self {
        Self::MutableBinding(identifier)
    }

    pub fn new_ignoring() -> Self {
        Self::Ignoring
    }
}
