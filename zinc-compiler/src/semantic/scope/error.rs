//!
//! The semantic analyzer scope error.
//!

use crate::lexical::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ItemUndeclared(String),
    ItemRedeclared(String, Location),
    ItemIsNotNamespace(String),
}
