//!
//! The boolean literal.
//!

use crate::lexical::token::lexeme::literal::boolean::Boolean as LexicalBooleanLiteral;
use crate::lexical::token::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub location: Location,
    pub inner: LexicalBooleanLiteral,
}

impl Literal {
    pub fn new(location: Location, inner: LexicalBooleanLiteral) -> Self {
        Self { location, inner }
    }
}

impl Into<bool> for Literal {
    fn into(self) -> bool {
        self.inner.into()
    }
}
