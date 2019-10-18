//!
//! The interpreter scope error.
//!

use failure::Fail;

use parser::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared variable '{}'", _0)]
    UndeclaredVariable(String),
    #[fail(display = "mutating an immutable variable '{}'", _0)]
    MutatingImmutableVariable(String),
    #[fail(display = "undeclared type '{}'", _0)]
    UndeclaredType(String),
    #[fail(display = "redeclared item '{}'", _0)]
    RedeclaredItem(String),

    #[fail(
        display = "the value being assigned has type '{}', but expected '{}'",
        _0, _1
    )]
    AssignmentInvalidType(TypeVariant, TypeVariant),
    #[fail(display = "addressing the primitive type variable '{}'", _0)]
    AddressingPrimitiveTypeVariable(String),

    #[fail(display = "index {} is out of range of '{}'", _0, _1)]
    ArrayIndexOutOfRange(usize, String),
    #[fail(display = "array '{}' cannot be addressed with tuple field {}", _0, _1)]
    AddressArrayAsTuple(String, usize),
    #[fail(
        display = "array '{}' cannot be addressed with structure field '{}'",
        _0, _1
    )]
    AddressArrayAsStructure(String, String),

    #[fail(display = "tuple field {} does not exist in '{}'", _0, _1)]
    TupleFieldNotExists(usize, String),
    #[fail(display = "tuple '{}' cannot be addressed with array index {}", _0, _1)]
    AccessTupleAsArray(String, usize),
    #[fail(
        display = "tuple '{}' cannot be addressed with structure field '{}'",
        _0, _1
    )]
    AccessTupleAsStructure(String, String),

    #[fail(display = "structure field '{}' does not exist in '{}'", _0, _1)]
    StructureFieldNotExists(String, String),
    #[fail(
        display = "structure '{}' cannot be addressed with array index {}",
        _0, _1
    )]
    AccessStructureAsArray(String, usize),
    #[fail(
        display = "structure '{}' cannot be addressed with tuple field {}",
        _0, _1
    )]
    AccessStructureAsTuple(String, usize),
}
