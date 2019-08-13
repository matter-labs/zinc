//!
//! The symbol lexical parser.
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

#[derive(Debug, Fail, Serialize)]
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
                b'{' => return Ok((size + 1, Symbol::BracketCurlyOpen)),
                b'}' => return Ok((size + 1, Symbol::BracketCurlyClose)),
                b'[' => return Ok((size + 1, Symbol::BracketSquareOpen)),
                b']' => return Ok((size + 1, Symbol::BracketSquareClose)),
                b'(' => return Ok((size + 1, Symbol::BracketRoundOpen)),
                b')' => return Ok((size + 1, Symbol::BracketRoundClose)),

                b'.' => return Ok((size + 1, Symbol::Dot)),
                b':' => return Ok((size + 1, Symbol::Colon)),
                b';' => return Ok((size + 1, Symbol::Semicolon)),
                b',' => return Ok((size + 1, Symbol::Comma)),

                b'+' => return Ok((size + 1, Symbol::ArithmeticAddition)),
                b'-' => return Ok((size + 1, Symbol::ArithmeticSubtractionOrArithmeticNegation)),
                b'*' => return Ok((size + 1, Symbol::ArithmeticMultiplication)),
                b'/' => return Ok((size + 1, Symbol::ArithmeticDivision)),
                b'%' => return Ok((size + 1, Symbol::ArithmeticRemainder)),
                b'\\' => return Ok((size + 1, Symbol::ArithmeticInversion)),

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
                b'=' => return Ok((size + 1, Symbol::ComparisonEqual)),
                _ => return Ok((size, Symbol::Assignment)),
            },
            State::Exclamation => match byte {
                b'=' => return Ok((size + 1, Symbol::ComparisonNotEqual)),
                _ => return Ok((size, Symbol::BooleanNot)),
            },
            State::Lesser => match byte {
                b'=' => return Ok((size + 1, Symbol::ComparisonLesserEqual)),
                _ => return Ok((size, Symbol::ComparisonLesser)),
            },
            State::Greater => match byte {
                b'=' => return Ok((size + 1, Symbol::ComparisonGreaterEqual)),
                _ => return Ok((size, Symbol::ComparisonGreater)),
            },
            State::And => match byte {
                b'&' => return Ok((size + 1, Symbol::BooleanAnd)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ))
                }
            },
            State::Or => match byte {
                b'|' => return Ok((size + 1, Symbol::BooleanOr)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ))
                }
            },
            State::Xor => match byte {
                b'^' => return Ok((size + 1, Symbol::BooleanXor)),
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
