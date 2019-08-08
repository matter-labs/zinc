//!
//! The integer lexical analyzer.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::IntegerLiteral;

pub enum State {
    Start,
    ZeroOrHexadecimal,
    Decimal,
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
    #[fail(display = "not an integer")]
    NotAnInteger,
    #[fail(
        display = "invalid decimal digit '{}' at position {} of '{}'",
        _0, _1, _2
    )]
    InvalidDecimalCharacter(char, usize, String),
    #[fail(
        display = "hexadecimal integer literals must start with '0x' and have at least one digit"
    )]
    InvalidHexadecimalFormat,
    #[fail(
        display = "invalid hexadecimal digit '{}' at position {} of '{}'",
        _0, _1, _2
    )]
    InvalidHexadecimalCharacter(char, usize, String),
}

impl Analyzer {
    pub fn analyze(mut self, bytes: &[u8]) -> Result<(usize, IntegerLiteral), Error> {
        let mut size = 0;
        while let Some(byte) = bytes.get(size) {
            let byte = *byte;

            match self.state {
                State::Start => {
                    if byte == b'0' {
                        self.state = State::ZeroOrHexadecimal;
                    } else if byte.is_ascii_digit() {
                        self.state = State::Decimal;
                    } else {
                        return Err(Error::NotAnInteger);
                    }
                }
                State::ZeroOrHexadecimal => {
                    if byte == b'x' {
                        self.state = State::Hexadecimal;
                    } else if byte.is_ascii_alphabetic() {
                        return Err(Error::InvalidDecimalCharacter(
                            char::from(byte),
                            size + 1,
                            String::from_utf8_lossy(&bytes[..=size]).to_string(),
                        ));
                    } else {
                        break;
                    }
                }
                State::Decimal => {
                    if byte.is_ascii_alphabetic() {
                        return Err(Error::InvalidDecimalCharacter(
                            char::from(byte),
                            size + 1,
                            String::from_utf8_lossy(&bytes[..=size]).to_string(),
                        ));
                    }

                    if !byte.is_ascii_digit() && byte != b'_' {
                        break;
                    }
                }
                State::Hexadecimal => {
                    if !byte.is_ascii_hexdigit() && byte != b'_' {
                        if byte.is_ascii_alphabetic() || size <= 2 {
                            return Err(Error::InvalidHexadecimalCharacter(
                                char::from(byte),
                                size + 1,
                                String::from_utf8_lossy(&bytes[..=size]).to_string(),
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
