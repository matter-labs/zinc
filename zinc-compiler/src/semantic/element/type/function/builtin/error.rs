//!
//! The semantic analyzer built-in function error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    Unknown {
        location: Location,
        function: String,
    },
    SpecifierMissing {
        location: Location,
        function: &'static str,
    },
    DebugArgumentCount {
        location: Location,
        expected: usize,
        found: usize,
    },
}
