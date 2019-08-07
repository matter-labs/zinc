//!
//! The integer lexical analyzer.
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
    #[fail(display = "invalid character '{}' at position {}", _0, _1)]
    InvalidDecimalCharacter(char, usize),
    #[fail(display = "hexadecimal integer literals must start with '0x'")]
    InvalidHexadecimalFormat,
    #[fail(display = "invalid character '{}' at position {}", _0, _1)]
    InvalidHexadecimalCharacter(char, usize),
}

impl Analyzer {
    pub fn analyze(mut self, bytes: &[u8]) -> Result<(usize, IntegerLiteral), Error> {
        let mut size = 0;
        while let Some(byte) = bytes.get(size) {
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
                        return Err(Error::InvalidDecimalCharacter(char::from(byte), size + 1));
                    }

                    if !byte.is_ascii_digit() && byte != b'_' {
                        break;
                    }
                }
                State::HexadecimalSymbolX => {
                    if byte == b'x' {
                        self.state = State::Hexadecimal;
                    } else {
                        return Err(Error::InvalidHexadecimalFormat);
                    }
                }
                State::Hexadecimal => {
                    if !byte.is_ascii_hexdigit() && byte != b'_' {
                        if byte.is_ascii_alphabetic() {
                            return Err(Error::InvalidHexadecimalCharacter(
                                char::from(byte),
                                size + 1,
                            ));
                        } else {
                            break;
                        }
                    }
                }
            }

            size += 1;
        }

        let literal = IntegerLiteral::from(&bytes[..size]);
        Ok((size, literal))
    }
}
