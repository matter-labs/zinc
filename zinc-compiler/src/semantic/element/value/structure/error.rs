//!
//! The semantic analyzer structure value element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    FieldDoesNotExist {
        type_identifier: String,
        field_name: String,
    },
    FieldExpected {
        type_identifier: String,
        position: usize,
        expected: String,
        found: String,
    },
    FieldInvalidType {
        type_identifier: String,
        field_name: String,
        expected: String,
        found: String,
    },
    FieldOutOfRange {
        type_identifier: String,
        expected: usize,
        found: usize,
    },
}
