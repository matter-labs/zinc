//!
//! The interpreter scope error.
//!

use failure::Fail;

use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared variable '{}'", _0)]
    UndeclaredVariable(String),
    #[fail(display = "redeclared variable '{}'", _0)]
    RedeclaredVariable(String),
    #[fail(display = "mutating an immutable variable '{}'", _0)]
    MutatingImmutableVariable(String),

    #[fail(display = "undeclared type '{}'", _0)]
    UndeclaredType(String),
    #[fail(display = "redeclared type '{}'", _0)]
    RedeclaredType(String),

    #[fail(
        display = "the value being assigned has type '{}', but expected '{}'",
        _0, _1
    )]
    AssignmentInvalidType(TypeVariant, TypeVariant),
    #[fail(display = "addressing the primitive type variable '{}'", _0)]
    AddressingPrimitiveTypeVariable(String),

    #[fail(display = "index {} is out of range of '{}'", _0, _1)]
    ArrayIndexOutOfRange(usize, String),
    #[fail(display = "array '{}' has no tuple field {}", _0, _1)]
    ArrayAccessingTupleField(String, usize),
    #[fail(display = "array '{}' has no structure field '{}'", _0, _1)]
    ArrayAccessingStructureField(String, String),

    #[fail(display = "tuple field {} is out of range of '{}'", _0, _1)]
    TupleFieldOutOfRange(usize, String),
    #[fail(
        display = "tuple '{}' cannot be indexed as array with index {}",
        _0, _1
    )]
    TupleIndexing(String, usize),
    #[fail(display = "tuple '{}' has no structure field '{}'", _0, _1)]
    TupleAccessingStructureField(String, String),

    #[fail(display = "structure field '{}' does not exist in '{}'", _0, _1)]
    StructureFieldNotExists(String, String),
    #[fail(
        display = "structure '{}' cannot be indexed as array with index {}",
        _0, _1
    )]
    StructureIndexing(String, usize),
    #[fail(
        display = "structure '{}' cannot be accessed with tuple field {}",
        _0, _1
    )]
    StructureAccessingWithTupleField(String, usize),
}
