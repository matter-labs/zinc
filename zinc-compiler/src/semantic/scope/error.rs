//!
//! The semantic analyzer scope error.
//!

use failure::Fail;

use crate::semantic::PlaceDescriptor;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared item '{}'", _0)]
    ItemUndeclared(String),
    #[fail(display = "redeclared item '{}'", _0)]
    ItemRedeclared(String),

    #[fail(display = "type '{}' cannot be accessed with '{}'", _0, _1)]
    InvalidDescriptor(String, PlaceDescriptor),
    #[fail(display = "array index {} is out of range of type '{}'", _0, _1)]
    ArrayIndexOutOfRange(usize, String),
    #[fail(display = "tuple field {} does not exist in '{}'", _0, _1)]
    TupleFieldDoesNotExist(usize, String),
    #[fail(display = "structure field {} does not exist in '{}'", _0, _1)]
    StructureFieldDoesNotExist(String, String),
}
