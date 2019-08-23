//!
//! The integer literal parser.
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

#[derive(Debug, Fail, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
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

pub fn parse(bytes: &[u8]) -> Result<(usize, IntegerLiteral), Error> {
    let mut state = State::Start;
    let mut size = 0;
    let mut value = Vec::with_capacity(40);

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => {
                if byte == b'0' {
                    value.push(byte);
                    size += 1;
                    state = State::ZeroOrHexadecimal;
                } else if byte.is_ascii_digit() {
                    value.push(byte);
                    size += 1;
                    state = State::Decimal;
                } else {
                    return Err(Error::NotAnInteger);
                }
            }
            State::ZeroOrHexadecimal => {
                if byte == b'x' {
                    size += 1;
                    state = State::Hexadecimal;
                } else if byte.is_ascii_alphabetic() {
                    return Err(Error::InvalidDecimalCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ));
                } else {
                    return Ok((size, IntegerLiteral::decimal(value)));
                }
            }
            State::Decimal => {
                if byte.is_ascii_digit() {
                    value.push(byte);
                    size += 1;
                } else if byte.is_ascii_alphabetic() {
                    return Err(Error::InvalidDecimalCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ));
                } else if !byte.is_ascii_digit() && byte != b'_' {
                    return Ok((size, IntegerLiteral::decimal(value)));
                }
            }
            State::Hexadecimal => {
                if byte.is_ascii_hexdigit() {
                    value.push(byte);
                    size += 1;
                } else if byte != b'_' && (byte.is_ascii_alphabetic() || size <= 2) {
                    return Err(Error::InvalidHexadecimalCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ));
                } else {
                    return Ok((size, IntegerLiteral::hexadecimal(value)));
                }
            }
        }
    }

    Err(Error::UnexpectedEnd)
}
