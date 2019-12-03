//!
//! The semantic analyzer place element error.
//!

use failure::Fail;

use crate::semantic::ConstantError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "index constant: {}", _0)]
    IndexConstant(ConstantError),
    #[fail(display = "tuple access constant: {}", _0)]
    TupleAccessConstant(ConstantError),
}
