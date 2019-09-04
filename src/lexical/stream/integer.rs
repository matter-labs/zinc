//!
//! The integer literal parser.
//!

use std::str;

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::IntegerLiteral;

pub enum State {
    Start,
    ZeroOrHexadecimal,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, Fail, Serialize, Clone, PartialEq)]
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
                    value.clear();
                    state = State::Hexadecimal;
                } else if byte.is_ascii_alphabetic() {
                    return Err(Error::InvalidDecimalCharacter(
                        char::from(byte),
                        size + 1,
                        unsafe { str::from_utf8_unchecked(&bytes[..=size]) }.to_owned(),
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
                        unsafe { str::from_utf8_unchecked(&bytes[..=size]) }.to_owned(),
                    ));
                } else if byte == b'_' {
                    size += 1;
                } else {
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
                        unsafe { str::from_utf8_unchecked(&bytes[..=size]) }.to_owned(),
                    ));
                } else if byte == b'_' {
                    size += 1;
                } else {
                    return Ok((size, IntegerLiteral::hexadecimal(value)));
                }
            }
        }
    }

    Err(Error::UnexpectedEnd)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;
    use crate::lexical::IntegerLiteral;

    #[test]
    fn ok_decimal_zero() {
        let input = b"0\n";
        let expected = Ok((1, IntegerLiteral::decimal(b"0".to_vec())));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_decimal() {
        let input = b"666\n";
        let expected = Ok((3, IntegerLiteral::decimal(b"666".to_vec())));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_hexadecimal_lowercase() {
        let input = b"0xDEAD_666_BEEF\n";
        let expected = Ok((15, IntegerLiteral::hexadecimal(b"DEAD666BEEF".to_vec())));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_hexadecimal_uppercase() {
        let input = b"0xdead_666_beef\n";
        let expected = Ok((15, IntegerLiteral::hexadecimal(b"dead666beef".to_vec())));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_unexpected_end() {
        let input = b"555";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
