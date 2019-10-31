//!
//! The lexical string parser.
//!

use std::str;

use failure::Fail;

pub enum State {
    DoubleQuoteOpen,
    Character,
    EscapedCharacter,
}

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
    #[fail(display = "not a string")]
    NotAString,
}

pub fn parse(input: &str) -> Result<(usize, String), Error> {
    let mut state = State::DoubleQuoteOpen;
    let mut size = 0;
    let mut value = String::with_capacity(64);

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::DoubleQuoteOpen => match character {
                '\"' => {
                    size += 1;
                    state = State::Character;
                }
                _ => return Err(Error::NotAString),
            },
            State::Character => match character {
                '\"' => {
                    size += 1;
                    return Ok((size, value));
                }
                '\\' => {
                    size += 1;
                    state = State::EscapedCharacter;
                }
                _ => {
                    value.push(character);
                    size += 1;
                }
            },
            State::EscapedCharacter => {
                value.push(character);
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
        let input = "\"some string\"";
        let expected = Ok((13, "some string".to_owned()));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn error_unexpected_end() {
        let input = "\"some string";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn error_not_a_string() {
        let input = "no double quote here";
        let expected = Err(Error::NotAString);
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
