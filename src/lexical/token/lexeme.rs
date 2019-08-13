//!
//! The token lexeme.
//!

use std::fmt;

use serde_derive::Serialize;

use crate::lexical::Comment;
use crate::lexical::Identifier;
use crate::lexical::Keyword;
use crate::lexical::Literal;
use crate::lexical::Symbol;

#[derive(Debug, Serialize, PartialEq)]
pub enum Lexeme {
    Keyword(Keyword),
    Identifier(Identifier),
    Literal(Literal),
    Symbol(Symbol),
    Comment(Comment),
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lexeme::Literal(literal) => write!(f, "{}", literal),
            Lexeme::Symbol(symbol) => write!(f, "{}", symbol),
            Lexeme::Identifier(identifier) => write!(f, "{}", identifier),
            _ => write!(f, "{:?}", self),
        }
    }
}
