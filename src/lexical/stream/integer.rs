//!
//! The lexical analyzer.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::IntegerLiteral;

pub enum State {
    Start,
    Decimal,
    HexadecimalSymbolX,
    Hexadecimal,
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

#[derive(Default)]
pub struct Analyzer {
    state: State,
}

#[derive(Debug, Fail, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "invalid character at position {}", _0)]
    InvalidDecimalCharacter(usize),
    #[fail(display = "invalid character at position {}", _0)]
    InvalidHexadecimalCharacter(usize),
}

impl Analyzer {
    pub fn analyze(mut self, bytes: &[u8], start: usize) -> Result<(usize, IntegerLiteral), Error> {
        let mut end = start;
        while let Some(byte) = bytes.get(end) {
            let byte = *byte;

            match self.state {
                State::Start => {
                    if byte == b'0' {
                        self.state = State::HexadecimalSymbolX;
                    } else {
                        self.state = State::Decimal;
                    }
                }
                State::Decimal => {
                    if byte.is_ascii_alphabetic() {
                        return Err(Error::InvalidDecimalCharacter(end - start));
                    }

                    if !byte.is_ascii_digit() && byte != b'_' {
                        break;
                    }
                }
                State::HexadecimalSymbolX => {
                    if byte == b'x' {
                        self.state = State::Hexadecimal;
                    } else {
                        return Err(Error::InvalidHexadecimalCharacter(end - start));
                    }
                }
                State::Hexadecimal => {
                    if !byte.is_ascii_hexdigit() && byte != b'_' {
                        if byte.is_ascii_alphabetic() {
                            return Err(Error::InvalidDecimalCharacter(end - start));
                        } else {
                            break;
                        }
                    }
                }
            }
            end += 1;
        }

        let literal = IntegerLiteral::from(&bytes[start..end]);
        Ok((end, literal))
    }
}
