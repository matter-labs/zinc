//!
//! The string literal.
//!

use crate::lexical::token::lexeme::literal::string::String as LexicalStringLiteral;
use crate::lexical::token::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub location: Location,
    pub inner: LexicalStringLiteral,
}

impl Literal {
    pub fn new(location: Location, inner: LexicalStringLiteral) -> Self {
        Self { location, inner }
    }
}

impl Into<::std::string::String> for Literal {
    fn into(self) -> ::std::string::String {
        self.inner.into()
    }
}
