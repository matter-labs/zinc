//!
//! Casting error.
//!

use failure::Fail;

use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(
        display = "semantic.casting from invalid type: from '{}' to '{}'",
        _0, _1
    )]
    FromInvalidType(TypeVariant, TypeVariant),
    #[fail(
        display = "semantic.casting to invalid type: from '{}' to '{}'",
        _0, _1
    )]
    ToInvalidType(TypeVariant, TypeVariant),
    #[fail(
        display = "semantic.casting to lesser bitlength: from {} to {}",
        _0, _1
    )]
    DataLossPossible(usize, usize),
}
