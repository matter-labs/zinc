//!
//! The semantic analyzer `use` statement error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer `use` statement error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The element after the `use` keyword must be a path to an item.
    ExpectedPath {
        /// The invalid element location in the code.
        location: Location,
        /// The stringified invalid element.
        found: String,
    },
}
