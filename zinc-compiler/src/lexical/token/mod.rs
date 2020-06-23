//!
//! The lexical token.
//!

pub mod lexeme;
pub mod location;

use std::fmt;

use self::lexeme::Lexeme;
use self::location::Location;

///
/// The lexical token, which represents the smallest logical piece of the source code.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The inner lexeme with the token data.
    pub lexeme: Lexeme,
    /// The token location in the source code.
    pub location: Location,
}

impl Token {
    ///
    /// Creates a new token with a known location.
    ///
    pub fn new(lexeme: Lexeme, location: Location) -> Self {
        Self { lexeme, location }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.location, self.lexeme)
    }
}
