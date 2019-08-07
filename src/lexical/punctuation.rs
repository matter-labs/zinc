//!
//! The punctuation lexeme.
//!

use std::convert::TryFrom;

#[derive(Debug)]
pub enum Punctuation {
    Colon,
    Semicolon,
    Comma,
}

impl TryFrom<u8> for Punctuation {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            b':' => Punctuation::Colon,
            b';' => Punctuation::Semicolon,
            b',' => Punctuation::Comma,
            another => return Err(another),
        })
    }
}
