//!
//! The semantic analyzer integer value element error.
//!

use failure::Fail;

use crate::semantic::CastingError;
use crate::semantic::Type;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "operand types mismatch: '{}' and '{}'", _0, _1)]
    OperandTypesMismatch(Type, Type),
    #[fail(display = "integer bitlength is too big for negation: {}", _0)]
    Negation(usize),
    #[fail(display = "casting: {}", _0)]
    Casting(CastingError),
}
