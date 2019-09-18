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
    Dot,
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

pub fn parse(input: &str) -> Result<(usize, Symbol), Error> {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::Start => match character {
                '{' => return Ok((size + 1, Symbol::BracketCurlyLeft)),
                '}' => return Ok((size + 1, Symbol::BracketCurlyRight)),
                '[' => return Ok((size + 1, Symbol::BracketSquareLeft)),
                ']' => return Ok((size + 1, Symbol::BracketSquareRight)),
                '(' => return Ok((size + 1, Symbol::ParenthesisLeft)),
                ')' => return Ok((size + 1, Symbol::ParenthesisRight)),

                ':' => return Ok((size + 1, Symbol::Colon)),
                ';' => return Ok((size + 1, Symbol::Semicolon)),
                ',' => return Ok((size + 1, Symbol::Comma)),

                '+' => return Ok((size + 1, Symbol::Plus)),
                '-' => return Ok((size + 1, Symbol::Minus)),
                '*' => return Ok((size + 1, Symbol::Asterisk)),
                '/' => return Ok((size + 1, Symbol::Slash)),
                '%' => return Ok((size + 1, Symbol::Percent)),
                '\\' => return Ok((size + 1, Symbol::Backslash)),

                '.' => {
                    size += 1;
                    state = State::Dot;
                }
                '=' => {
                    size += 1;
                    state = State::Equal;
                }
                '!' => {
                    size += 1;
                    state = State::Exclamation;
                }
                '<' => {
                    size += 1;
                    state = State::Lesser;
                }
                '>' => {
                    size += 1;
                    state = State::Greater;
                }
                '&' => {
                    size += 1;
                    state = State::And;
                }
                '|' => {
                    size += 1;
                    state = State::Or;
                }
                '^' => {
                    size += 1;
                    state = State::Xor;
                }

                _ => return Err(Error::NotASymbol),
            },
            State::Equal => match character {
                '=' => return Ok((size + 1, Symbol::DoubleEquals)),
                _ => return Ok((size, Symbol::Equals)),
            },
            State::Exclamation => match character {
                '=' => return Ok((size + 1, Symbol::ExclamationMarkEquals)),
                _ => return Ok((size, Symbol::ExclamationMark)),
            },
            State::Lesser => match character {
                '=' => return Ok((size + 1, Symbol::LesserThanEquals)),
                _ => return Ok((size, Symbol::LesserThan)),
            },
            State::Greater => match character {
                '=' => return Ok((size + 1, Symbol::GreaterThanEquals)),
                _ => return Ok((size, Symbol::GreaterThan)),
            },
            State::Dot => match character {
                '.' => return Ok((size + 1, Symbol::DoubleDot)),
                _ => return Ok((size, Symbol::Dot)),
            },
            State::And => match character {
                '&' => return Ok((size + 1, Symbol::DoubleAmpersand)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        character,
                        size + 1,
                        input[..=size].to_owned(),
                    ))
                }
            },
            State::Or => match character {
                '|' => return Ok((size + 1, Symbol::DoubleVerticalBar)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        character,
                        size + 1,
                        input[..=size].to_owned(),
                    ))
                }
            },
            State::Xor => match character {
                '^' => return Ok((size + 1, Symbol::DoubleCircumflex)),
                _ => {
                    return Err(Error::InvalidCharacter(
                        character,
                        size + 1,
                        input[..=size].to_owned(),
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
        let input = "==";
        let expected = Ok((2, Symbol::DoubleEquals));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_unexpected_end() {
        let input = "|";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_not_a_symbol() {
        let input = "5";
        let expected = Err(Error::NotASymbol);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_invalid_character() {
        let input = "|5";
        let expected = Err(Error::InvalidCharacter('5', 2, "|5".to_owned()));
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
