//!
//! The semantic analyzer tuple index element.
//!

use std::fmt;

use zinc_lexical::Location;

///
/// A tuple expression field identifier, e.g. `(1, 2, 3).2`.
///
#[derive(Debug, Clone, PartialEq)]
pub struct TupleIndex {
    /// The location in the code.
    pub location: Location,
    /// The tuple index value, that is, the tuple element position.
    pub value: usize,
}

impl TupleIndex {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(location: Location, value: usize) -> Self {
        Self { location, value }
    }
}

impl fmt::Display for TupleIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
