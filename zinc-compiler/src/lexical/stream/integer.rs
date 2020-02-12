//!
//! The lexical integer literal parser.
//!

use std::str;

use failure::Fail;

use crate::lexical::token::lexeme::literal::integer::Integer;

pub enum State {
    Start,
    ZeroOrHexadecimal,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, Fail, Clone, PartialEq)]
pub enum Error {
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
    #[fail(display = "not an integer")]
    NotAnInteger,
    #[fail(
        display = "invalid decimal character '{}' at position {} of '{}'",
        _0, _1, _2
    )]
    InvalidDecimalCharacter(char, usize, String),
    #[fail(
        display = "invalid hexadecimal character '{}' at position {} of '{}'",
        _0, _1, _2
    )]
    InvalidHexadecimalCharacter(char, usize, String),
    #[fail(display = "hexadecimal literal must have at least one digit after '0x'")]
    EmptyHexadecimalLiteral,
}

pub fn parse(input: &str) -> Result<(usize, Integer), Error> {
    let mut state = State::Start;
    let mut size = 0;
    let mut value = String::with_capacity(40);

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::Start => {
                if character == '0' {
                    value.push(character);
                    size += 1;
                    state = State::ZeroOrHexadecimal;
                } else if character.is_ascii_digit() {
                    value.push(character);
                    size += 1;
                    state = State::Decimal;
                } else {
                    return Err(Error::NotAnInteger);
                }
            }
            State::ZeroOrHexadecimal => {
                if character == 'x' {
                    size += 1;
                    value.clear();
                    state = State::Hexadecimal;
                } else if character.is_ascii_alphabetic() {
                    return Err(Error::InvalidDecimalCharacter(
                        character,
                        size + 1,
                        input[..=size].to_owned(),
                    ));
                } else {
                    return Ok((size, Integer::new_decimal(value)));
                }
            }
            State::Decimal => {
                if character.is_ascii_digit() {
                    value.push(character);
                    size += 1;
                } else if character.is_ascii_alphabetic() {
                    return Err(Error::InvalidDecimalCharacter(
                        character,
                        size + 1,
                        input[..=size].to_owned(),
                    ));
                } else if character == '_' {
                    size += 1;
                } else {
                    return Ok((size, Integer::new_decimal(value)));
                }
            }
            State::Hexadecimal => {
                if character.is_ascii_hexdigit() {
                    value.push(character.to_ascii_lowercase());
                    size += 1;
                } else if character != '_' && (character.is_ascii_alphabetic() || size <= 2) {
                    return Err(Error::InvalidHexadecimalCharacter(
                        character,
                        size + 1,
                        input[..=size].to_owned(),
                    ));
                } else if character == '_' {
                    size += 1;
                } else {
                    return Ok((size, Integer::new_hexadecimal(value)));
                }
            }
        }
    }

    match state {
        State::Start => Err(Error::UnexpectedEnd),
        State::ZeroOrHexadecimal => Ok((size, Integer::new_decimal(value))),
        State::Decimal => Ok((size, Integer::new_decimal(value))),
        State::Hexadecimal => {
            if !value.is_empty() {
                Ok((size, Integer::new_hexadecimal(value)))
            } else {
                Err(Error::EmptyHexadecimalLiteral)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;
    use crate::lexical::token::lexeme::literal::integer::Integer;

    #[test]
    fn ok_decimal_zero() {
        let input = "0";
        let expected = Ok((1, Integer::new_decimal("0".to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_decimal() {
        let input = "666";
        let expected = Ok((3, Integer::new_decimal("666".to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_hexadecimal_lowercase() {
        let input = "0xdead_666_beef";
        let expected = Ok((15, Integer::new_hexadecimal("dead666beef".to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_hexadecimal_uppercase() {
        let input = "0xDEAD_666_BEEF";
        let expected = Ok((15, Integer::new_hexadecimal("dead666beef".to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_hexadecimal_mixed_case() {
        let input = "0xdEaD_666_bEeF";
        let expected = Ok((15, Integer::new_hexadecimal("dead666beef".to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn err_unexpected_end() {
        let input = "";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn err_not_an_integer() {
        let input = "xyz";
        let expected = Err(Error::NotAnInteger);
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn err_invalid_decimal_character() {
        let input = "25x";
        let expected = Err(Error::InvalidDecimalCharacter('x', 3, "25x".to_owned()));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn err_invalid_hexadecimal_character() {
        let input = "0xABC_X";
        let expected = Err(Error::InvalidHexadecimalCharacter(
            'X',
            7,
            "0xABC_X".to_owned(),
        ));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn err_empty_hexadecimal_literal() {
        let input = "0x";
        let expected = Err(Error::EmptyHexadecimalLiteral);
        let result = parse(input);
        assert_eq!(result, expected);
    }
}
