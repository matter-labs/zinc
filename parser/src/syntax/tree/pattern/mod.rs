//!
//! The pattern.
//!

mod builder;
mod variant;

pub use self::builder::Builder;
pub use self::variant::Variant;

use std::fmt;

use crate::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub location: Location,
    pub variant: Variant,
}

impl Pattern {
    pub fn new(location: Location, variant: Variant) -> Self {
        Self { location, variant }
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}
