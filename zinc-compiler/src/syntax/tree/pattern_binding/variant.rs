//!
//! The binding pattern variant.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;

///
/// The binding pattern variant.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// An ordinar function argument, like `a: u8` or `mut a: u8`.
    Binding {
        /// The argument name.
        identifier: Identifier,
        /// If the argument variable is mutable.
        is_mutable: bool,
    },
    /// A wildcard function argument, like `_`.
    Wildcard,
    /// An object instance method argument, like `self` or `mut self`.
    SelfAlias {
        /// The `self` alias location.
        location: Location,
        /// If the instance is mutable.
        is_mutable: bool,
    },
}

impl Variant {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_binding(identifier: Identifier, is_mutable: bool) -> Self {
        Self::Binding {
            identifier,
            is_mutable,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_wildcard() -> Self {
        Self::Wildcard
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_self_alias(location: Location, is_mutable: bool) -> Self {
        Self::SelfAlias {
            location,
            is_mutable,
        }
    }
}
