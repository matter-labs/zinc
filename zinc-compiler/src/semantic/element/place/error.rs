//!
//! The semantic analyzer place element error.
//!

use failure::Fail;

use crate::semantic::IntegerConstantError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "index constant: {}", _0)]
    IndexConstant(IntegerConstantError),
}
