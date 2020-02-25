//!
//! The type.
//!

pub mod builder;
pub mod variant;

use crate::lexical::Location;

use self::variant::Variant;

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub location: Location,
    pub variant: Variant,
}

impl Type {
    pub fn new(location: Location, variant: Variant) -> Self {
        Self { location, variant }
    }
}
