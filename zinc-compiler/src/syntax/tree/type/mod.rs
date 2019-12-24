//!
//! The type.
//!

mod builder;
mod variant;

pub use self::builder::Builder;
pub use self::variant::Variant;

use crate::lexical::Location;

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
