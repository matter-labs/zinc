//!
//! The binding pattern variant.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    Binding {
        identifier: Identifier,
        is_mutable: bool,
    },
    Wildcard,
    SelfAlias {
        location: Location,
        is_mutable: bool,
    },
}

impl Variant {
    pub fn new_binding(identifier: Identifier, is_mutable: bool) -> Self {
        Self::Binding {
            identifier,
            is_mutable,
        }
    }

    pub fn new_wildcard() -> Self {
        Self::Wildcard
    }

    pub fn new_self_alias(location: Location, is_mutable: bool) -> Self {
        Self::SelfAlias {
            location,
            is_mutable,
        }
    }
}
