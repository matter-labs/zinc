//!
//! The token.
//!

mod lexeme;
mod location;

pub use self::lexeme::Lexeme;
pub use self::location::Location;

use std::fmt;

#[derive(Debug)]
pub struct Token {
    lexeme: Lexeme,
    location: Location,
}

impl Token {
    pub fn new(lexeme: Lexeme, location: Location) -> Self {
        Self { lexeme, location }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?}", self.location, self.lexeme)
    }
}
