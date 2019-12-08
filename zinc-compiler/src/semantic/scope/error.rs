//!
//! The semantic analyzer scope error.
//!

use failure::Fail;

use crate::semantic::PlaceDescriptor;
use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared item '{}'", _0)]
    ItemUndeclared(String),
    #[fail(display = "redeclared item '{}'", _0)]
    ItemRedeclared(String),
    #[fail(display = "item '{}' is not a variable", _0)]
    ItemNotVariable(String),
    #[fail(display = "item '{}' is not a type", _0)]
    ItemNotType(String),
    #[fail(display = "item '{}' is not an enumeration", _0)]
    ItemNotEnumeration(String),

    #[fail(display = "type '{}' cannot be accessed with '{}'", _0, _1)]
    InvalidDescriptor(Type, PlaceDescriptor),
    #[fail(display = "array index {} is out of range of type '{}'", _0, _1)]
    ArrayIndexOutOfRange(usize, Type),
    #[fail(display = "tuple field {} does not exist in '{}'", _0, _1)]
    TupleFieldDoesNotExist(usize, Type),
    #[fail(display = "structure field {} does not exist in '{}'", _0, _1)]
    StructureFieldDoesNotExist(String, Type),
    #[fail(display = "enumeration variant '{}' does not exist in '{}'", _0, _1)]
    EnumerationVariantDoesNotExist(String, String),
}
