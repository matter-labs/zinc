//!
//! The symbol parser.
//!

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

#[derive(Debug, Fail, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "not an symbol")]
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

                b'=' => state = State::Equal,
                b'!' => state = State::Exclamation,
                b'<' => state = State::Lesser,
                b'>' => state = State::Greater,
                b'&' => state = State::And,
                b'|' => state = State::Or,
                b'^' => state = State::Xor,

                _ => return Err(Error::NotASymbol),
            },
            State::Equal => match byte {
                b'=' => return Ok((size + 1, Symbol::DoubleEquals)),
                _ => return Ok((size, Symbol::Equals)),
            },
            State::Exclamation => match byte {
                b'=' => return Ok((size + 1, Symbol::ExclamationEquals)),
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
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ))
                }
            },
            State::Or => match byte {
                b'|' => return Ok((size + 1, Symbol::DoubleVerticalBar)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ))
                }
            },
            State::Xor => match byte {
                b'^' => return Ok((size + 1, Symbol::DoubleCircumflex)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ))
                }
            },
        }

        size += 1;
    }

    let symbol = Symbol::from(&bytes[..size]);
    Ok((size, symbol))
}
