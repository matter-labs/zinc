//!
//! The semantic analyzer structure value element error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "field '{}' already exists", _0)]
    FieldAlreadyExists(String),
    #[fail(display = "structure field '{}' does not exist in '{}'", _0, _1)]
    FieldDoesNotExistInStructure(String, String), // TODO
}
