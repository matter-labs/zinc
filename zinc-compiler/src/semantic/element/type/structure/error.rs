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
}
