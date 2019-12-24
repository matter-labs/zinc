//!
//! The semantic analyzer place element error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "only raw identifiers can appear in path: {}", _0)]
    PathDescripted(String),
}
