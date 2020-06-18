//!
//! The semantic attribute error.
//!

use crate::lexical::token::location::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    Unknown { location: Location, found: String },
}
