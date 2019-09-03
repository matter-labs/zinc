//!
//! The string parser.
//!

use std::str;

use failure::Fail;
use serde_derive::Serialize;

pub enum State {
    DoubleQuoteOpen,
    Character,
    EscapedCharacter,
}

#[derive(Debug, Fail, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
    #[fail(display = "not a string")]
    NotAString,
}

pub fn parse(bytes: &[u8]) -> Result<(usize, Vec<u8>), Error> {
    let mut state = State::DoubleQuoteOpen;
    let mut size = 0;
    let mut value = Vec::with_capacity(64);

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::DoubleQuoteOpen => match byte {
                b'\"' => {
                    size += 1;
                    state = State::Character;
                }
                _ => return Err(Error::NotAString),
            },
            State::Character => match byte {
                b'\"' => {
                    size += 1;
                    return Ok((size, value));
                }
                b'\\' => {
                    size += 1;
                    state = State::EscapedCharacter;
                }
                _ => {
                    value.push(byte);
                    size += 1;
                }
            },
            State::EscapedCharacter => {
                value.push(byte);
                size += 1;
                state = State::Character;
            }
        }
    }

    Err(Error::UnexpectedEnd)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;

    #[test]
    fn ok() {
        let input = b"\"some string\"";
        let expected = Ok((13, b"some string".to_vec()));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_unexpected_end() {
        let input = b"\"some string";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_not_a_string() {
        let input = b"no double quote here";
        let expected = Err(Error::NotAString);
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
