//!
//! The integer literal.
//!

use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
use crate::lexical::token::location::Location;

///
/// The integer literal.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    /// The location of the syntax construction.
    pub location: Location,
    /// The inner lexical literal.
    pub inner: LexicalIntegerLiteral,
}

impl Literal {
    ///
    /// Creates a new literal value.
    ///
    pub fn new(location: Location, inner: LexicalIntegerLiteral) -> Self {
        Self { location, inner }
    }
}
