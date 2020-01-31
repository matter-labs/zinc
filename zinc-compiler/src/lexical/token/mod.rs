//!
//! The lexical token.
//!

pub mod lexeme;
pub mod location;

use std::fmt;

use self::lexeme::Lexeme;
use self::location::Location;

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
