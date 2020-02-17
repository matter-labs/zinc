//!
//! The semantic analyzer scope error.
//!

use crate::lexical::Location;

use failure::Fail;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "undeclared item '{}'", _0)]
    ItemUndeclared(String),
    #[fail(display = "redeclared item '{}' at {}", _0, _1)]
    ItemRedeclared(String, Location),
    #[fail(display = "item is not a namespace '{}'", _0)]
    ItemIsNotNamespace(String),
}
