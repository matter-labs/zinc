//!
//! The integer literal.
//!

use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
use crate::lexical::token::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub location: Location,
    pub inner: LexicalIntegerLiteral,
}

impl Literal {
    pub fn new(location: Location, inner: LexicalIntegerLiteral) -> Self {
        Self { location, inner }
    }
}
