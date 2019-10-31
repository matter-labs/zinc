//!
//! Inference error.
//!

use failure::Fail;

use crate::lexical::IntegerLiteral;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "literal '{}' is larger than {} bits", _0, _1)]
    LiteralTooLarge(IntegerLiteral, usize),
}
