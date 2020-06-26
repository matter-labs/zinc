//!
//! The identifier.
//!

pub mod builder;

use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::location::Location;

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
    pub fn is_self(&self) -> bool {
        self.name == Keyword::SelfLowercase.to_string()
    }
}
