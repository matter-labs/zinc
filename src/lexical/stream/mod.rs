//!
//! The lexeme stream.
//!

mod integer;
mod word;

pub use self::integer::Analyzer as IntegerAnalyzer;
pub use self::integer::Error as IntegerAnalyzerError;
pub use self::word::Analyzer as WordAnalyzer;

use std::iter::Iterator;

use crate::lexical::Alphabet;
use crate::lexical::Delimiter;
use crate::lexical::Error;
use crate::lexical::Identifier;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Punctuation;

pub struct Stream {
    input: Vec<u8>,
    position: usize,
    last_error: Option<Error>,
}

impl Stream {
    pub fn new(input: Vec<u8>) -> Self {
        Self {
            input,
            position: 0,
            last_error: None,
        }
    }

    pub fn last_error(&self) -> Option<&Error> {
        self.last_error.as_ref()
    }
}

impl Iterator for Stream {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(byte) = self.input.get(self.position) {
            let byte = *byte;

            if !Alphabet::contains(byte) {
                self.last_error = Some(Error::Forbidden(char::from(byte)));
                return None;
            }

            if byte.is_ascii_whitespace() {
                self.position += 1;
                continue;
            }

            if Punctuation::can_be(byte) {
                self.position += 1;
                return Some(Lexeme::Punctuation(Punctuation::from(byte)));
            }

            if Delimiter::can_be(byte) {
                self.position += 1;
                return Some(Lexeme::Delimiter(Delimiter::from(byte)));
            }

            if byte.is_ascii_digit() {
                match IntegerAnalyzer::default().analyze(self.input.as_slice(), self.position) {
                    Ok((index, integer)) => {
                        self.position = index;
                        return Some(Lexeme::Literal(Literal::Integer(integer)));
                    }
                    Err(error) => {
                        self.last_error = Some(Error::InvalidIntegerLiteral(error));
                        return None;
                    }
                }
            }

            if Identifier::can_start_with(byte) {
                let (index, lexeme) =
                    WordAnalyzer::default().analyze(self.input.as_slice(), self.position);
                self.position = index;
                return Some(lexeme);
            } else {
                break;
            }
        }

        None
    }
}
