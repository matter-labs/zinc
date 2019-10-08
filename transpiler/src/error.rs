//!
//! The transpiler error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "fake")]
    Fake,
}
