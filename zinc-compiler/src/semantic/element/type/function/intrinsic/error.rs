//!
//! The semantic analyzer intrinsic function error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer intrinsic function error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Tried to call an unknown function with the `!` specifier.
    Unknown {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: String,
    },
    /// The intrinsic functions can only be called with the `!` specifier.
    SpecifierMissing {
        /// The error location data.
        location: Location,
        /// The function identifier.
        function: &'static str,
    },
    /// The `dbg!(...)` function argument count does not match the number of placeholders in the format string.
    DebugArgumentCount {
        /// The error location data.
        location: Location,
        /// The number of expected function arguments including the format string.
        expected: usize,
        /// The number of actual function arguments including the format string.
        found: usize,
    },
}
