//!
//! The semantic analyzer scope error.
//!

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared item '{}'", _0)]
    ItemUndeclared(String),
    #[fail(display = "redeclared item '{}'", _0)]
    ItemRedeclared(String),
    #[fail(display = "item is not a namespace '{}'", _0)]
    ItemIsNotNamespace(String),
}
