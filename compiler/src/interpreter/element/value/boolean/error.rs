//!
//! The interpreter boolean error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "inner allocation: {}", _0)]
    InnerAllocation(String),
    #[fail(display = "inner operation '{}': {}", _0, _1)]
    InnerOperation(&'static str, String),
}
