//!
//! The interpreter value error.
//!

use failure::Fail;

use crate::interpreter::BooleanError;
use crate::interpreter::IntegerError;
use crate::interpreter::Value;

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
    #[fail(display = "operand types mismatch: [{}] and [{}]", _0, _1)]
    OperandTypesMismatch(Value, Value),
    #[fail(
        display = "operator '{}' expected a boolean value, but got [{}]",
        _0, _1
    )]
    ExpectedBoolean(&'static str, Value),
    #[fail(
        display = "operator '{}' expected an integer value, but got [{}]",
        _0, _1
    )]
    ExpectedInteger(&'static str, Value),
}
