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
    /// Built-in function like `dbg!` or `assert!` where the `!` specifier is required.
    BuiltIn,
    /// Object method call where the first `self` argument must be temporarily stored.
    Method {
        /// The `self` instance, for which the method is called.
        instance: Box<Element>,
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
    pub fn new_method(instance: Element) -> Self {
        Self::Method {
            instance: Box::new(instance),
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
