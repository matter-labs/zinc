//!
//! The `dbg!` intrinsic function error.
//!

use crate::lexical::token::location::Location;

///
/// The `dbg!` intrinsic function error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The `dbg!(...)` function argument count does not match the number of placeholders in the format string.
    ArgumentCount {
        /// The error location data.
        location: Location,
        /// The number of expected function arguments including the format string.
        expected: usize,
        /// The number of actual function arguments including the format string.
        found: usize,
    },
}
