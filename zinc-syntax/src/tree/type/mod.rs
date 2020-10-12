//!
//! The type.
//!

pub mod builder;
pub mod variant;

use zinc_lexical::Location;

use self::variant::Variant;

///
/// The type.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    /// The location of the syntax construction.
    pub location: Location,
    /// The type variant.
    pub variant: Variant,
}

impl Type {
    ///
    /// Creates a type.
    ///
    pub fn new(location: Location, variant: Variant) -> Self {
        Self { location, variant }
    }
}
