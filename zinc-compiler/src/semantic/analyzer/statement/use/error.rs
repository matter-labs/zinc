//!
//! The semantic analyzer `use` statement error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedPath { location: Location, found: String },
}
