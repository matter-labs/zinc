//!
//! The match pattern.
//!

pub mod builder;
pub mod variant;

use zinc_lexical::Location;

use self::variant::Variant;

///
/// The match pattern.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    /// The location of the syntax construction.
    pub location: Location,
    /// The pattern match variant.
    pub variant: Variant,
}

impl Pattern {
    ///
    /// Creates a match pattern.
    ///
    pub fn new(location: Location, variant: Variant) -> Self {
        Self { location, variant }
    }
}
