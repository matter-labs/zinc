//!
//! The semantic analyzer contract value element error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    FieldDoesNotExist {
        location: Location,
        type_identifier: String,
        field_name: String,
    },
    FieldExpected {
        location: Location,
        type_identifier: String,
        position: usize,
        expected: String,
        found: String,
    },
    FieldInvalidType {
        location: Location,
        type_identifier: String,
        field_name: String,
        expected: String,
        found: String,
    },
    FieldOutOfRange {
        location: Location,
        type_identifier: String,
        expected: usize,
        found: usize,
    },
}
