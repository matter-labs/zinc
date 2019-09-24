//!
//! The generator error.
//!

use failure::Fail;
use serde_derive::Serialize;

#[derive(Debug, Fail, Serialize, PartialEq)]
pub enum Error {
    #[fail(display = "Unreachable")]
    Unreachable,
}
