//!
//! The semantic analyzer array value element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    FieldDoesNotExist {
        type_identifier: String,
        field_index: usize,
    },
}
