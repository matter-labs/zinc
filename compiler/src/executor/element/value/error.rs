//!
//! The interpreter value error.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::executor::Integer;
use crate::executor::Value;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, Serialize, PartialEq)]
pub enum Error {
    #[fail(
        display = "operand types mismatch: [{}] and [{}] have different types",
        _0, _1
    )]
    OperandTypesMismatch(Value, Value),
    #[fail(display = "integer literal is larger than {} bits", _0)]
    IntegerLiteralIsTooLarge(usize),
    #[fail(display = "casting to invalid type: from [{}] to '{}'", _0, _1)]
    CastingToInvalidType(Integer, TypeVariant),
    #[fail(display = "casting to lesser bitlength: from {} to {}", _0, _1)]
    CastingToLesserOrEqualBitlength(usize, usize),
}
