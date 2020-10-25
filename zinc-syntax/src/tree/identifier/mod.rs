//!
//! The identifier.
//!

pub mod builder;

use zinc_lexical::Keyword;
use zinc_lexical::Location;

///
/// The identifier.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    /// The location of the syntax construction.
    pub location: Location,
    /// The identifier string contents.
    pub name: String,
}

impl Identifier {
    ///
    /// Creates an identifier.
    ///
    pub fn new(location: Location, name: String) -> Self {
        Self { location, name }
    }

    ///
    /// Checks if the identifier is a `self` alias keyword.
    ///
    pub fn is_self_lowercase(&self) -> bool {
        self.name == Keyword::SelfLowercase.to_string()
    }
}
