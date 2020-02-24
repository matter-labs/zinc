//!
//! The binding pattern variant.
//!

use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Binding(Identifier),
    MutableBinding(Identifier),
    Wildcard,
}

impl Variant {
    pub fn new_binding(identifier: Identifier) -> Self {
        Self::Binding(identifier)
    }

    pub fn new_mutable_binding(identifier: Identifier) -> Self {
        Self::MutableBinding(identifier)
    }

    pub fn new_wildcard() -> Self {
        Self::Wildcard
    }
}
