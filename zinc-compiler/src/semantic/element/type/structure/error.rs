//!
//! The semantic analyzer structure type element error.
//!

use zinc_lexical::Location;

///
/// The semantic analyzer structure type element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// A field with the same name occurs for the second time.
    DuplicateField {
        /// The duplicate field location.
        location: Location,
        /// The structure type name.
        type_identifier: String,
        /// The duplicate field name.
        field_name: String,
    },
    /// The structure expected generics, but got none.
    ExpectedGenerics {
        /// The type initializer location.
        location: Location,
        /// The structure type name.
        type_identifier: String,
        /// The number of expected generics.
        expected: usize,
    },
    /// The structure did not expect generics, but got some.
    UnexpectedGenerics {
        /// The type initializer location.
        location: Location,
        /// The structure type name.
        type_identifier: String,
    },
    /// The structure expected different number of generic arguments.
    InvalidGenericsNumber {
        /// The type initializer location.
        location: Location,
        /// The structure type name.
        type_identifier: String,
        /// The number of expected generics.
        expected: usize,
        /// The number of found generics.
        found: usize,
    },
}
