//!
//! The function call type.
//!

use std::mem;

use crate::semantic::element::Element;

///
/// Describes some function call specifics.
///
#[derive(Debug)]
pub enum Type {
    /// Default function call without any extra behavior.
    Default,
    /// Intrinsic function like `dbg!` where the `!` specifier is required.
    MacroLike,
    /// Object method call where the first `self` argument must be temporarily stored.
    Method {
        /// The `self` instance, for which the method is called.
        instance: Box<Element>,
        /// Whether the instance, for which the method is called, is mutable.
        is_mutable: bool,
    },
}

impl Default for Type {
    fn default() -> Self {
        Self::Default
    }
}

impl Type {
    ///
    /// A shortcut constructor.
    ///
    pub fn new_method(instance: Element, is_mutable: bool) -> Self {
        Self::Method {
            instance: Box::new(instance),
            is_mutable,
        }
    }
}

impl Type {
    ///
    /// Resembles the `Option::take` behavior, where `self` is replaced with `T::default`.
    ///
    pub fn take(&mut self) -> Self {
        mem::take(self)
    }
}
