//!
//! The semantic analyzer structure value element error.
//!

#[derive(Debug, PartialEq)]
pub enum Error {
    FieldAlreadyExists(String, String),
    FieldDoesNotExist(String, String),
    FieldInvalidType(String, String, String, String),
}
