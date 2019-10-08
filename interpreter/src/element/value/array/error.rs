//!
//! The interpreter element array value error.
//!

use failure::Fail;

use parser::TypeVariant;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "pushing a value of type '{}', but expected '{}'", _0, _1)]
    PushingInvalidType(TypeVariant, TypeVariant),
}
