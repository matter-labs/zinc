//!
//! The lexical integer literal parser.
//!

use std::str;

use crate::lexical::token::lexeme::literal::integer::Integer;

pub enum State {
    Start,
    ZeroOrHexadecimal,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotAnInteger,
    EmptyHexadecimalBody,
    ExpectedOneOfDecimal { found: char, offset: usize },
    ExpectedOneOfHexadecimal { found: char, offset: usize },
    UnexpectedEnd,
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
                    return Err(Error::ExpectedOneOfDecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_decimal(value)));
                }
            }
            State::Decimal => {
                if character.is_ascii_digit() {
                    value.push(character);
                    size += 1;
                } else if character.is_ascii_alphabetic() {
                    return Err(Error::ExpectedOneOfDecimal {
                        found: character,
                        offset: size,
                    });
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
                    return Err(Error::ExpectedOneOfHexadecimal {
                        found: character,
                        offset: size,
                    });
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
                Err(Error::EmptyHexadecimalBody)
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
        let expected = Ok((input.len(), Integer::new_decimal(input.to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_decimal() {
        let input = "666";
        let expected = Ok((input.len(), Integer::new_decimal(input.to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_hexadecimal_lowercase() {
        let input = "0xdead_666_beef";
        let filtered = "dead666beef";
        let expected = Ok((input.len(), Integer::new_hexadecimal(filtered.to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_hexadecimal_uppercase() {
        let input = "0xDEAD_666_BEEF";
        let filtered = "dead666beef";
        let expected = Ok((input.len(), Integer::new_hexadecimal(filtered.to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_hexadecimal_mixed_case() {
        let input = "0xdEaD_666_bEeF";
        let filtered = "dead666beef";
        let expected = Ok((input.len(), Integer::new_hexadecimal(filtered.to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_not_an_integer() {
        let input = "xyz";
        let expected = Err(Error::NotAnInteger);
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_empty_hexadecimal_body() {
        let input = "0x";
        let expected = Err(Error::EmptyHexadecimalBody);
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_one_of_decimal() {
        let input = "25x";
        let expected = Err(Error::ExpectedOneOfDecimal {
            found: 'x',
            offset: 2,
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_one_of_hexadecimal() {
        let input = "0xABC_X";
        let expected = Err(Error::ExpectedOneOfHexadecimal {
            found: 'X',
            offset: 6,
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_unexpected_end() {
        let input = "";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(result, expected);
    }
}
