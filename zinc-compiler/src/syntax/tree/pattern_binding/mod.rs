//!
//! The binding pattern.
//!

pub mod builder;
pub mod variant;

use crate::lexical::Location;
use crate::syntax::tree::r#type::Type;

use self::variant::Variant;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub location: Location,
    pub variant: Variant,
    pub r#type: Type,
}

impl Pattern {
    pub fn new(location: Location, variant: Variant, r#type: Type) -> Self {
        Self {
            location,
            variant,
            r#type,
        }
    }
}
