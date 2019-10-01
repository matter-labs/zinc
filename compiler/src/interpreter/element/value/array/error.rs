//!
//! The interpreter array error.
//!

use failure::Fail;

use crate::syntax::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "pushing an invalid type '{}', but expected '{}'", _0, _1)]
    PushingInvalidType(TypeVariant, TypeVariant),
}
