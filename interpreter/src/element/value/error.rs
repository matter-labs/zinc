//!
//! The interpreter element value error.
//!

use failure::Fail;

use crate::element::ArrayError;
use crate::element::BooleanError;
use crate::element::IntegerError;
use crate::element::StructureError;
use crate::element::Value;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "inner allocation: {}", _0)]
    InnerAllocation(String),
    #[fail(display = "inner operation: {}", _0)]
    InnerOperation(&'static str, String),
    #[fail(display = "boolean: {}", _0)]
    Boolean(BooleanError),
    #[fail(display = "integer: {}", _0)]
    Integer(IntegerError),
    #[fail(display = "array: {}", _0)]
    Array(ArrayError),
    #[fail(display = "structure: {}", _0)]
    Structure(StructureError),
    #[fail(display = "operand types mismatch: '{}' and '{}'", _0, _1)]
    OperandTypesMismatch(Value, Value),
    #[fail(
        display = "operator '{}' expected a boolean value, but got '{}'",
        _0, _1
    )]
    ExpectedBoolean(&'static str, Value),
    #[fail(
        display = "operator '{}' expected an integer value, but got '{}'",
        _0, _1
    )]
    ExpectedInteger(&'static str, Value),
}
