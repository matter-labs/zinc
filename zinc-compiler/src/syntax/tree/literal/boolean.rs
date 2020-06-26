//!
//! The boolean literal.
//!

use crate::lexical::token::lexeme::literal::boolean::Boolean as LexicalBooleanLiteral;
use crate::lexical::token::location::Location;

///
/// The boolean literal.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    /// The location of the syntax construction.
    pub location: Location,
    /// The inner lexical literal.
    pub inner: LexicalBooleanLiteral,
}

impl Literal {
    ///
    /// Creates a new literal value.
    ///
    pub fn new(location: Location, inner: LexicalBooleanLiteral) -> Self {
        Self { location, inner }
    }
}

impl Into<bool> for Literal {
    fn into(self) -> bool {
        self.inner.into()
    }
}
