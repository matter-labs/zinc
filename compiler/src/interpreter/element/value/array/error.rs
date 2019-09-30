//!
//! The interpreter array error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "unimplemented")]
    Unimplemented,
}
