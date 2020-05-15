//!
//! The semantic analyzer contract type element error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    DuplicateField {
        location: Location,
        type_identifier: String,
        field_name: String,
    },
}
