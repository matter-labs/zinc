//!
//! The semantic analyzer element integer value error.
//!

use failure::Fail;

use crate::semantic::InferenceError;
use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "operand types mismatch: '{}' and '{}'", _0, _1)]
    OperandTypesMismatch(TypeVariant, TypeVariant),
    #[fail(display = "semantic.inference: {}", _0)]
    Inference(InferenceError),
}
