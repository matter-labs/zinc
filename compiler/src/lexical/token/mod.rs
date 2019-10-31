//!
//! The lexical token.
//!

mod lexeme;
mod location;

pub use self::lexeme::BooleanLiteral;
pub use self::lexeme::Comment;
pub use self::lexeme::Identifier;
pub use self::lexeme::IdentifierError;
pub use self::lexeme::IntegerLiteral;
pub use self::lexeme::Keyword;
pub use self::lexeme::KeywordError;
pub use self::lexeme::Lexeme;
pub use self::lexeme::Literal;
pub use self::lexeme::StringLiteral;
pub use self::lexeme::Symbol;
pub use self::location::Location;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub lexeme: Lexeme,
    pub location: Location,
}

impl Token {
    pub fn new(lexeme: Lexeme, location: Location) -> Self {
        Self { lexeme, location }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.location, self.lexeme)
    }
}
