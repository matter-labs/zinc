//!
//! Inference error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "literal is larger than {} bits", _0)]
    LiteralTooLarge(usize),
}
