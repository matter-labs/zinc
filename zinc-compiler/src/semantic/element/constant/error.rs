//!
//! The semantic analyzer constant element error.
//!

use failure::Fail;
use num_bigint::BigInt;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "constant '{}' is too big to be used as an index", _0)]
    ConstantTooBigForIndex(BigInt),
}
