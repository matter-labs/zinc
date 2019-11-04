//!
//! The semantic analyzer element boolean value error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "unimplemented")]
    _Unimplemented,
}
