//!
//! The match pattern.
//!

pub mod builder;
pub mod variant;

use crate::lexical::token::location::Location;

use self::variant::Variant;

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
