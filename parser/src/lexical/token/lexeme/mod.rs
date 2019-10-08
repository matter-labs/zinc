//!
//! The lexical token lexeme.
//!

mod comment;
mod identifier;
mod keyword;
mod literal;
mod symbol;

pub use self::comment::Comment;
pub use self::identifier::Error as IdentifierError;
pub use self::identifier::Identifier;
pub use self::keyword::Error as KeywordError;
pub use self::keyword::Keyword;
pub use self::literal::Boolean as BooleanLiteral;
pub use self::literal::Integer as IntegerLiteral;
pub use self::literal::Literal;
pub use self::literal::String as StringLiteral;
pub use self::symbol::Symbol;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
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
            Self::Keyword(keyword) => write!(f, "{}", keyword),
            Self::Identifier(identifier) => write!(f, "{}", identifier),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Symbol(symbol) => write!(f, "{}", symbol),
            Self::Comment(comment) => write!(f, "{}", comment),
        }
    }
}
