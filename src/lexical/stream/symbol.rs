//!
//! The symbol parser.
//!

use std::str;

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::Symbol;

pub enum State {
    Start,
    Equal,
    Exclamation,
    Lesser,
    Greater,
    And,
    Or,
    Xor,
}

#[derive(Debug, Fail, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
    #[fail(display = "not a symbol")]
    NotASymbol,
    #[fail(display = "invalid character '{}' at position {} of '{}'", _0, _1, _2)]
    InvalidCharacter(char, usize, String),
}

pub fn parse(bytes: &[u8]) -> Result<(usize, Symbol), Error> {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => match byte {
                b'{' => return Ok((size + 1, Symbol::BracketCurlyLeft)),
                b'}' => return Ok((size + 1, Symbol::BracketCurlyRight)),
                b'[' => return Ok((size + 1, Symbol::BracketSquareLeft)),
                b']' => return Ok((size + 1, Symbol::BracketSquareRight)),
                b'(' => return Ok((size + 1, Symbol::ParenthesisLeft)),
                b')' => return Ok((size + 1, Symbol::ParenthesisRight)),

                b'.' => return Ok((size + 1, Symbol::Dot)),
                b':' => return Ok((size + 1, Symbol::Colon)),
                b';' => return Ok((size + 1, Symbol::Semicolon)),
                b',' => return Ok((size + 1, Symbol::Comma)),

                b'+' => return Ok((size + 1, Symbol::Plus)),
                b'-' => return Ok((size + 1, Symbol::Minus)),
                b'*' => return Ok((size + 1, Symbol::Asterisk)),
                b'/' => return Ok((size + 1, Symbol::Slash)),
                b'%' => return Ok((size + 1, Symbol::Percent)),
                b'\\' => return Ok((size + 1, Symbol::Backslash)),

                b'=' => {
                    size += 1;
                    state = State::Equal;
                }
                b'!' => {
                    size += 1;
                    state = State::Exclamation;
                }
                b'<' => {
                    size += 1;
                    state = State::Lesser;
                }
                b'>' => {
                    size += 1;
                    state = State::Greater;
                }
                b'&' => {
                    size += 1;
                    state = State::And;
                }
                b'|' => {
                    size += 1;
                    state = State::Or;
                }
                b'^' => {
                    size += 1;
                    state = State::Xor;
                }

                _ => return Err(Error::NotASymbol),
            },
            State::Equal => match byte {
                b'=' => return Ok((size + 1, Symbol::DoubleEquals)),
                _ => return Ok((size, Symbol::Equals)),
            },
            State::Exclamation => match byte {
                b'=' => return Ok((size + 1, Symbol::ExclamationMarkEquals)),
                _ => return Ok((size, Symbol::ExclamationMark)),
            },
            State::Lesser => match byte {
                b'=' => return Ok((size + 1, Symbol::LesserThanEquals)),
                _ => return Ok((size, Symbol::LesserThan)),
            },
            State::Greater => match byte {
                b'=' => return Ok((size + 1, Symbol::GreaterThanEquals)),
                _ => return Ok((size, Symbol::GreaterThan)),
            },
            State::And => match byte {
                b'&' => return Ok((size + 1, Symbol::DoubleAmpersand)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        unsafe { str::from_utf8_unchecked(&bytes[..=size]) }.to_owned(),
                    ))
                }
            },
            State::Or => match byte {
                b'|' => return Ok((size + 1, Symbol::DoubleVerticalBar)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        unsafe { str::from_utf8_unchecked(&bytes[..=size]) }.to_owned(),
                    ))
                }
            },
            State::Xor => match byte {
                b'^' => return Ok((size + 1, Symbol::DoubleCircumflex)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        unsafe { str::from_utf8_unchecked(&bytes[..=size]) }.to_owned(),
                    ))
                }
            },
        }
    }

    Err(Error::UnexpectedEnd)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;
    use crate::lexical::Symbol;

    #[test]
    fn ok() {
        let input = b"==";
        let expected = Ok((2, Symbol::DoubleEquals));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_unexpected_end() {
        let input = b"|";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_not_a_symbol() {
        let input = b"5";
        let expected = Err(Error::NotASymbol);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_invalid_character() {
        let input = b"|5";
        let expected = Err(Error::InvalidCharacter('5', 2, "|5".to_owned()));
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
