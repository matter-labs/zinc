//!
//! The delimiter lexeme.
//!

#[derive(Debug)]
pub enum Delimiter {
    BracketCurlyOpen,
    BracketCurlyClose,
    BracketSquareOpen,
    BracketSquareClose,
    BracketRoundOpen,
    BracketRoundClose,
}

impl Delimiter {
    pub fn can_be(byte: u8) -> bool {
        byte == b'{' || byte == b'}' || byte == b'[' || byte == b']' || byte == b'(' || byte == b')'
    }
}

impl From<u8> for Delimiter {
    fn from(byte: u8) -> Self {
        match byte {
            b'{' => Delimiter::BracketCurlyOpen,
            b'}' => Delimiter::BracketCurlyClose,
            b'[' => Delimiter::BracketSquareOpen,
            b']' => Delimiter::BracketSquareClose,
            b'(' => Delimiter::BracketRoundOpen,
            b')' => Delimiter::BracketRoundClose,
            _ => panic!("Invalid delimiter"),
        }
    }
}
