//!
//! The semantic analyzer scope error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ItemUndeclared {
        location: Location,
        name: String,
    },
    ItemRedeclared {
        location: Location,
        name: String,
        reference: Option<Location>,
    },
    ItemNotNamespace {
        location: Location,
        name: String,
    },
    ContractRedeclared {
        location: Location,
        name: String,
        reference: Option<Location>,
    },
}
