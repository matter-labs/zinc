//!
//! The binding pattern.
//!

mod builder;
mod variant;

pub use self::builder::Builder;
pub use self::variant::Variant;

use crate::lexical::Location;
use crate::syntax::Type;

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
