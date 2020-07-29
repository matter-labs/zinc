//!
//! The semantic analyzer structure value element error.
//!

use crate::lexical::token::location::Location;

///
/// The semantic analyzer structure value element error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The structure type appeared in the code without the structure literal with fields.
    NotInitialized {
        /// The error location data.
        location: Location,
        /// The stringified uninitiliazed structure type.
        type_identifier: String,
    },
    /// The provided field name does not exist in the structure type.
    FieldDoesNotExist {
        /// The error location data.
        location: Location,
        /// The stringified structure type.
        type_identifier: String,
        /// The name of the invalid field.
        field_name: String,
    },
    /// A provided field name does not match the one in the structure type at the same position.
    FieldExpected {
        /// The error location data.
        location: Location,
        /// The stringified structure type.
        type_identifier: String,
        /// The position of the erroneous field.
        position: usize,
        /// The name of the expected field.
        expected: String,
        /// The name of the invalid field, which was actually found.
        found: String,
    },
    /// A provided field type does not match the one in the structure type.
    FieldInvalidType {
        /// The error location data.
        location: Location,
        /// The stringified structure type.
        type_identifier: String,
        /// The erroneous field name.
        field_name: String,
        /// The expected type for the field.
        expected: String,
        /// The invalid type, which was actually found.
        found: String,
    },
    /// The number of provided fields is bigger than the expected one.
    FieldOutOfRange {
        /// The error location data.
        location: Location,
        /// The stringified structure type.
        type_identifier: String,
        /// The expected number of structure fields.
        expected: usize,
        /// The position of the provided structure field.
        found: usize,
    },
}
