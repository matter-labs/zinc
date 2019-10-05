//!
//! The interpreter structure error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "the field with name '{}' already exists", _0)]
    FieldAlreadyExists(String),
}
