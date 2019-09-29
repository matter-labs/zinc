//!
//! The interpreter value error.
//!

use failure::Fail;

use crate::interpreter::Integer;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "inner allocation: {}", _0)]
    InnerAllocation(String),
    #[fail(display = "inner operation '{}': {}", _0, _1)]
    InnerOperation(&'static str, String),
    #[fail(display = "literal is larger than {} bits", _0)]
    LiteralTooLarge(usize),
    #[fail(display = "operand types mismatch: [{}] and [{}]", _0, _1)]
    OperandTypesMismatch(Integer, Integer),
    #[fail(display = "casting to invalid type: from [{}] to '{}'", _0, _1)]
    CastingToInvalidType(Integer, TypeVariant),
    #[fail(display = "casting to lesser bitlength: from {} to {}", _0, _1)]
    CastingToLesserOrEqualBitlength(usize, usize),
}
