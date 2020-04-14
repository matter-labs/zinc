//!
//! The semantic analyzer `mod` statement error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotFound { location: Location, name: String },
}
