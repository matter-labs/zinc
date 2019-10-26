//!
//! The interpreter scope error.
//!

use failure::Fail;

use parser::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared item '{}'", _0)]
    UndeclaredItem(String),
    #[fail(display = "redeclared item '{}'", _0)]
    RedeclaredItem(String),

    #[fail(display = "mutating an immutable object '{}'", _0)]
    MutatingImmutable(String),
    #[fail(
        display = "the value being assigned has type '{}', but expected '{}'",
        _0, _1
    )]
    AssignmentInvalidType(TypeVariant, TypeVariant),
    #[fail(display = "addressing the primitive type variable '{}'", _0)]
    AddressingPrimitiveTypeVariable(String),

    #[fail(display = "index {} is out of range of '{}'", _0, _1)]
    ArrayIndexOutOfRange(usize, String),
    #[fail(display = "array '{}' cannot be accessed with tuple field {}", _0, _1)]
    ArrayAccessAsTuple(String, usize),
    #[fail(
        display = "array '{}' cannot be accessed with structure field '{}'",
        _0, _1
    )]
    ArrayAccessAsStructure(String, String),

    #[fail(display = "tuple field {} does not exist in '{}'", _0, _1)]
    TupleFieldNotExists(usize, String),
    #[fail(display = "tuple '{}' cannot be accessed with array index {}", _0, _1)]
    TupleAccessAsArray(String, usize),
    #[fail(
        display = "tuple '{}' cannot be accessed with structure field '{}'",
        _0, _1
    )]
    TupleAccessAsStructure(String, String),

    #[fail(display = "structure field '{}' does not exist in '{}'", _0, _1)]
    StructureFieldNotExists(String, String),
    #[fail(
        display = "structure '{}' cannot be accessed with array index {}",
        _0, _1
    )]
    StructureAccessAsArray(String, usize),
    #[fail(
        display = "structure '{}' cannot be accessed with tuple field {}",
        _0, _1
    )]
    StructureAccessAsTuple(String, usize),
}
