//!
//! The type.
//!

mod builder;
mod variant;

pub use self::builder::Builder;
pub use self::variant::Variant;

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Location;

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub struct Type {
    pub location: Location,
    pub variant: Variant,
}

impl Type {
    pub fn new(location: Location, variant: Variant) -> Self {
        Self { location, variant }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.variant)
    }
}
