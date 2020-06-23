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

///
/// The smallest logical piece of the source code.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Lexeme {
    /// A keyword, like `if` or `const`.
    Keyword(Keyword),
    /// An identifier, like `value` or `data`.
    Identifier(Identifier),
    /// A literal, like `true`, `42`, or `"message"`.
    Literal(Literal),
    /// A symbol, like `(` or `+=`.
    Symbol(Symbol),
    /// A comment, like `// text` or `/* text */`.
    Comment(Comment),
    /// An end of file, which is returned by the token stream when it reaches the end of a file.
    Eof,
}

impl fmt::Display for Lexeme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
