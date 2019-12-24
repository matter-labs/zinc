//!
//! The match pattern.
//!

mod builder;
mod variant;

pub use self::builder::Builder;
pub use self::variant::Variant;

use crate::lexical::Location;

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
