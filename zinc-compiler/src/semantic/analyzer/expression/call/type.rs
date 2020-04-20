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
    /// Normal function call without any extra behavior
    Normal,
    /// Built-in function like `dbg!` or `assert!` where the `!` specifier is required
    BuiltIn,
    /// Object method call where the first `self` argument must be temporarily stored
    Method { instance: Element },
}

impl Default for Type {
    fn default() -> Self {
        Self::Normal
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
