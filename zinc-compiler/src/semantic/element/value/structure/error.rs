//!
//! The semantic analyzer structure value element error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "field '{}' already exists in '{}'", _0, _1)]
    FieldAlreadyExists(String, String),
    #[fail(display = "structure field '{}' does not exist in '{}'", _0, _1)]
    FieldDoesNotExist(String, String),
}
