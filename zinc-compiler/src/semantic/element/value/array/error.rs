//!
//! The semantic analyzer array value element error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "pushing a value of type '{}' into an array of '{}'", _0, _1)]
    InvalidType(String, String),
}
