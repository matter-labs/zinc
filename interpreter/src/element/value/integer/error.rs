//!
//! The interpreter element integer value error.
//!

use failure::Fail;

use parser::TypeVariant;
use semantic::CastingError;
use semantic::InferenceError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "inner allocation: {}", _0)]
    InnerAllocation(String),
    #[fail(display = "inner operation '{}': {}", _0, _1)]
    InnerOperation(&'static str, String),
    #[fail(display = "literal is larger than {} bits", _0)]
    LiteralTooLarge(usize),
    #[fail(display = "operand types mismatch: '{}' and '{}'", _0, _1)]
    OperandTypesMismatch(TypeVariant, TypeVariant),
    #[fail(display = "inference: {}", _0)]
    Inference(InferenceError),
    #[fail(display = "casting: {}", _0)]
    Casting(CastingError),
}
