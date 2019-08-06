//!
//! The punctuation lexeme.
//!

#[derive(Debug)]
pub enum Punctuation {
    Colon,
    Semicolon,
    Comma,
}

impl Punctuation {
    pub fn can_be(byte: u8) -> bool {
        byte == b':' || byte == b';' || byte == b','
    }
}

impl From<u8> for Punctuation {
    fn from(byte: u8) -> Self {
        match byte {
            b':' => Punctuation::Colon,
            b';' => Punctuation::Semicolon,
            b',' => Punctuation::Comma,
            _ => panic!("Invalid punctuation"),
        }
    }
}
