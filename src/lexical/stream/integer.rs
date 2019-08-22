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
    #[fail(
        display = "decimal literal '{}' is too large: the max value is '{}'",
        _0, _1
    )]
    DecimalTooLong(String, String),
    #[fail(
        display = "hexadecimal literal '{}' is too large ({} digits): only 32 hexdigits allowed with the 32th <= 7",
        _0, _1
    )]
    HexadecimalTooLong(String, usize),
}

pub fn parse(bytes: &[u8]) -> Result<(usize, IntegerLiteral), Error> {
    const MAX_HEXADECIMAL_LENGTH: usize = 32;
    const MAX_DECIMAL: &[u8] = b"85070591730234615865843651857942052864";

    let mut state = State::Start;
    let mut size = 0;
    let mut value = Vec::with_capacity(40);

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => {
                if byte == b'0' {
                    state = State::ZeroOrHexadecimal;
                } else if byte.is_ascii_digit() {
                    value.push(byte);
                    state = State::Decimal;
                } else {
                    return Err(Error::NotAnInteger);
                }
            }
            State::ZeroOrHexadecimal => {
                if byte == b'x' {
                    state = State::Hexadecimal;
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
                if byte.is_ascii_digit() {
                    value.push(byte);
                }
                if value.len() > MAX_DECIMAL.len()
                    || (value.len() == MAX_DECIMAL.len() && value.as_slice() > MAX_DECIMAL)
                {
                    return Err(Error::DecimalTooLong(
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                        String::from_utf8_lossy(&MAX_DECIMAL).to_string(),
                    ));
                }

                if byte.is_ascii_alphabetic() {
                    return Err(Error::InvalidDecimalCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ));
                }

                if !byte.is_ascii_digit() && byte != b'_' {
                    return Ok((size, IntegerLiteral::decimal(value)));
                }
            }
            State::Hexadecimal => {
                if byte.is_ascii_hexdigit() {
                    value.push(byte);
                }
                if value.len() > MAX_HEXADECIMAL_LENGTH
                    || (value.len() == MAX_HEXADECIMAL_LENGTH && value[0] > b'7')
                {
                    return Err(Error::HexadecimalTooLong(
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                        value.len(),
                    ));
                }

                if !byte.is_ascii_hexdigit() && byte != b'_' {
                    if byte.is_ascii_alphabetic() || size <= 2 {
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

        size += 1;
    }

    unreachable!();
}
