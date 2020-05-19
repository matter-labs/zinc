//!
//! The semantic analyzer tuple index element.
//!

use std::fmt;

use crate::lexical::token::location::Location;

///
/// A tuple expression field identifier, e.g. `(1, 2, 3).2`.
///
#[derive(Debug, Clone, PartialEq)]
pub struct TupleIndex {
    pub location: Location,
    pub value: usize,
}

impl TupleIndex {
    pub fn new(location: Location, value: usize) -> Self {
        Self { location, value }
    }
}

impl fmt::Display for TupleIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
