//!
//! Casting error.
//!

use failure::Fail;

use parser::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "casting from invalid type: from '{}' to '{}'", _0, _1)]
    FromInvalidType(TypeVariant, TypeVariant),
    #[fail(display = "casting to invalid type: from '{}' to '{}'", _0, _1)]
    ToInvalidType(TypeVariant, TypeVariant),
    #[fail(display = "casting to lesser bitlength: from {} to {}", _0, _1)]
    DataLossPossible(usize, usize),
}
