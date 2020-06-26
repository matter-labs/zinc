//!
//! The binding pattern.
//!

pub mod builder;
pub mod variant;

use crate::lexical::token::location::Location;
use crate::syntax::tree::r#type::Type;

use self::variant::Variant;

///
/// The binding pattern.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    /// The location of the syntax construction.
    pub location: Location,
    /// The binding pattern variant.
    pub variant: Variant,
    /// The binding pattern type.
    pub r#type: Type,
}

impl Pattern {
    ///
    /// Creates a binding pattern.
    ///
    pub fn new(location: Location, variant: Variant, r#type: Type) -> Self {
        Self {
            location,
            variant,
            r#type,
        }
    }
}
