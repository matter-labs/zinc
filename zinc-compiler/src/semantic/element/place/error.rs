//!
//! The semantic analyzer place error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "'[]' operator expected an array as the first operand, but got '{}'",
        _0
    )]
    OperatorIndexFirstOperandExpectedArray(String), // TODO
    #[fail(
        display = "'[]' operator expected an integer as the second operand, but got '{}'",
        _0
    )]
    OperatorIndexSecondOperandExpectedInteger(String), // TODO
    #[fail(
        display = "'.' operator expected a tuple or structure as the first operand, but got '{}'",
        _0
    )]
    OperatorFieldFirstOperandExpectedTupleOrStructure(String), // TODO
    #[fail(display = "tuple field {} does not exist in '{}'", _0, _1)]
    FieldDoesNotExistInTuple(usize, String), // TODO
    #[fail(display = "structure field '{}' does not exist in '{}'", _0, _1)]
    FieldDoesNotExistInStructure(String, String), // TODO
}
