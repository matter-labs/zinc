//!
//! The string literal.
//!

use zinc_lexical::Location;
use zinc_lexical::StringLiteral as LexicalStringLiteral;

///
/// The string literal.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    /// The location of the syntax construction.
    pub location: Location,
    /// The inner lexical literal.
    pub inner: LexicalStringLiteral,
}

impl Literal {
    ///
    /// Creates a new literal value.
    ///
    pub fn new(location: Location, inner: LexicalStringLiteral) -> Self {
        Self { location, inner }
    }
}

impl Into<::std::string::String> for Literal {
    fn into(self) -> ::std::string::String {
        self.inner.into()
    }
}
