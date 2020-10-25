//!
//! The binding pattern variant.
//!

use crate::tree::identifier::Identifier;
use crate::tree::pattern_binding::Pattern as BindingPattern;

///
/// The binding pattern variant.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Variant {
    /// An ordinar variable binding, like `a` or `mut a`.
    Binding {
        /// The argument name.
        identifier: Identifier,
        /// If the argument variable is mutable.
        is_mutable: bool,
    },
    /// A variable list binding, like `(a, b, c)` or `(mut a, b, mut c)`.
    BindingList {
        /// The binding list elements.
        bindings: Vec<BindingPattern>,
    },
    /// A wildcard function argument, like `_`.
    Wildcard,
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
    pub fn new_binding_list(bindings: Vec<BindingPattern>) -> Self {
        Self::BindingList { bindings }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_wildcard() -> Self {
        Self::Wildcard
    }
}
