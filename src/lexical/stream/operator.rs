//!
//! The operator lexical parser.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::Operator;

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
    #[fail(display = "not an operator")]
    NotAnOperator,
    #[fail(display = "invalid character '{}' at position {} of '{}'", _0, _1, _2)]
    InvalidCharacter(char, usize, String),
}

pub fn parse(bytes: &[u8]) -> Result<(usize, Operator), Error> {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => match byte {
                b'(' => return Ok((size + 1, Operator::ParenthesisOpen)),
                b')' => return Ok((size + 1, Operator::ParenthesisClose)),
                b'.' => return Ok((size + 1, Operator::Dot)),
                b'+' => return Ok((size + 1, Operator::ArithmeticAddition)),
                b'-' => {
                    return Ok((
                        size + 1,
                        Operator::ArithmeticSubtractionOrArithmeticNegation,
                    ))
                }
                b'*' => return Ok((size + 1, Operator::ArithmeticMultiplication)),
                b'/' => return Ok((size + 1, Operator::ArithmeticDivision)),
                b'%' => return Ok((size + 1, Operator::ArithmeticRemainder)),
                b'\\' => return Ok((size + 1, Operator::ArithmeticInversion)),

                b'=' => state = State::Equal,
                b'!' => state = State::Exclamation,
                b'<' => state = State::Lesser,
                b'>' => state = State::Greater,
                b'&' => state = State::And,
                b'|' => state = State::Or,
                b'^' => state = State::Xor,

                _ => return Err(Error::NotAnOperator),
            },
            State::Equal => match byte {
                b'=' => return Ok((size + 1, Operator::ComparisonEqual)),
                _ => return Ok((size + 1, Operator::Assignment)),
            },
            State::Exclamation => match byte {
                b'=' => return Ok((size + 1, Operator::ComparisonNotEqual)),
                _ => return Ok((size + 1, Operator::BooleanNot)),
            },
            State::Lesser => match byte {
                b'=' => return Ok((size + 1, Operator::ComparisonLesserEqual)),
                _ => return Ok((size + 1, Operator::ComparisonLesser)),
            },
            State::Greater => match byte {
                b'=' => return Ok((size + 1, Operator::ComparisonGreaterEqual)),
                _ => return Ok((size + 1, Operator::ComparisonGreater)),
            },
            State::And => match byte {
                b'&' => return Ok((size + 1, Operator::BooleanAnd)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ))
                }
            },
            State::Or => match byte {
                b'|' => return Ok((size + 1, Operator::BooleanOr)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        char::from(byte),
                        size + 1,
                        String::from_utf8_lossy(&bytes[..=size]).to_string(),
                    ))
                }
            },
            State::Xor => match byte {
                b'^' => return Ok((size + 1, Operator::BooleanXor)),
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

    let operator = Operator::from(&bytes[..size]);
    Ok((size, operator))
}
