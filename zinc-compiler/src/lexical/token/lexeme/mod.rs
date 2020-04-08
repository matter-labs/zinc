//!
//! The lexical token lexeme.
//!

pub mod comment;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod symbol;

use std::fmt;

use self::comment::Comment;
use self::identifier::Identifier;
use self::keyword::Keyword;
use self::literal::Literal;
use self::symbol::Symbol;

#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme {
    Keyword(Keyword),
    Identifier(Identifier),
    Literal(Literal),
    Symbol(Symbol),
    Comment(Comment),
    Eof,
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Keyword(inner) => write!(f, "{}", inner),
            Self::Identifier(inner) => write!(f, "{}", inner),
            Self::Literal(inner) => write!(f, "{}", inner),
            Self::Symbol(inner) => write!(f, "{}", inner),
            Self::Comment(inner) => write!(f, "{}", inner),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
