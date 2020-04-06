//!
//! The lexical integer literal parser.
//!

use std::str;

use crate::lexical::token::lexeme::literal::integer::Integer;

pub enum State {
    Start,
    ZeroOrNotDecimal,
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    NotAnInteger,
    EmptyBinaryBody { offset: usize },
    EmptyOctalBody { offset: usize },
    EmptyHexadecimalBody { offset: usize },
    ExpectedOneOfBinary { found: char, offset: usize },
    ExpectedOneOfOctal { found: char, offset: usize },
    ExpectedOneOfDecimal { found: char, offset: usize },
    ExpectedOneOfHexadecimal { found: char, offset: usize },
    UnexpectedEnd,
}

///
/// Parses an integer literal.
///
/// Integer literals can be of four types:
///
/// 1. Binary
/// '0b101010'
///
/// 2. Octal
/// '52'
///
/// 3. Decimal
/// '42'
///
/// 4. Hexadecimal
/// '2a'
///
pub fn parse(input: &str) -> Result<(usize, Integer), Error> {
    let mut state = State::Start;
    let mut size = 0;
    let mut value = String::with_capacity(40);

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::Start => {
                if character == Integer::CHARACTER_ZERO {
                    value.push(character);
                    size += 1;
                    state = State::ZeroOrNotDecimal;
                } else if Integer::CHARACTERS_DECIMAL.contains(&character) {
                    value.push(character);
                    size += 1;
                    state = State::Decimal;
                } else {
                    return Err(Error::NotAnInteger);
                }
            }
            State::ZeroOrNotDecimal => {
                if character == Integer::CHARACTER_INITIAL_BINARY {
                    size += 1;
                    value.clear();
                    state = State::Binary;
                } else if character == Integer::CHARACTER_INITIAL_OCTAL {
                    size += 1;
                    value.clear();
                    state = State::Octal;
                } else if character == Integer::CHARACTER_INITIAL_HEXADECIMAL {
                    size += 1;
                    value.clear();
                    state = State::Hexadecimal;
                } else if character.is_ascii_alphanumeric() {
                    return Err(Error::ExpectedOneOfDecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_decimal(value)));
                }
            }
            State::Binary => {
                if Integer::CHARACTERS_BINARY.contains(&character) {
                    value.push(character.to_ascii_lowercase());
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character.is_ascii_alphanumeric() || size <= 2 {
                    return Err(Error::ExpectedOneOfBinary {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_binary(value)));
                }
            }
            State::Octal => {
                if Integer::CHARACTERS_OCTAL.contains(&character) {
                    value.push(character.to_ascii_lowercase());
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character.is_ascii_alphanumeric() || size <= 2 {
                    return Err(Error::ExpectedOneOfOctal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_octal(value)));
                }
            }
            State::Decimal => {
                if Integer::CHARACTERS_DECIMAL.contains(&character) {
                    value.push(character);
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character.is_ascii_alphanumeric() {
                    return Err(Error::ExpectedOneOfDecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_decimal(value)));
                }
            }
            State::Hexadecimal => {
                if Integer::CHARACTERS_HEXADECIMAL.contains(&character) {
                    value.push(character.to_ascii_lowercase());
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character.is_ascii_alphanumeric() || size <= 2 {
                    return Err(Error::ExpectedOneOfHexadecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_hexadecimal(value)));
                }
            }
        }
    }

    match state {
        State::Start => Err(Error::UnexpectedEnd),
        State::ZeroOrNotDecimal => Ok((size, Integer::new_decimal(value))),
        State::Binary => {
            if !value.is_empty() {
                Ok((size, Integer::new_binary(value)))
            } else {
                Err(Error::EmptyBinaryBody { offset: size })
            }
        }
        State::Octal => {
            if !value.is_empty() {
                Ok((size, Integer::new_octal(value)))
            } else {
                Err(Error::EmptyOctalBody { offset: size })
            }
        }
        State::Decimal => Ok((size, Integer::new_decimal(value))),
        State::Hexadecimal => {
            if !value.is_empty() {
                Ok((size, Integer::new_hexadecimal(value)))
            } else {
                Err(Error::EmptyHexadecimalBody { offset: size })
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
    fn ok_binary() {
        let input = "0b101010";
        let filtered = "101010";
        let expected = Ok((input.len(), Integer::new_binary(filtered.to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_octal() {
        let input = "0o42";
        let filtered = "42";
        let expected = Ok((input.len(), Integer::new_octal(filtered.to_owned())));
        let result = parse(input);
        assert_eq!(result, expected);
    }

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
    fn error_empty_binary_body() {
        let input = "0b";
        let expected = Err(Error::EmptyBinaryBody {
            offset: input.len(),
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_empty_octal_body() {
        let input = "0o";
        let expected = Err(Error::EmptyOctalBody {
            offset: input.len(),
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_empty_hexadecimal_body() {
        let input = "0x";
        let expected = Err(Error::EmptyHexadecimalBody {
            offset: input.len(),
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_one_of_binary() {
        let input = "0b101_2";
        let expected = Err(Error::ExpectedOneOfBinary {
            found: '2',
            offset: input.len() - 1,
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_one_of_octal() {
        let input = "0o147_8";
        let expected = Err(Error::ExpectedOneOfOctal {
            found: '8',
            offset: input.len() - 1,
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_one_of_decimal() {
        let input = "25x";
        let expected = Err(Error::ExpectedOneOfDecimal {
            found: 'x',
            offset: input.len() - 1,
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_one_of_hexadecimal() {
        let input = "0xABC_X";
        let expected = Err(Error::ExpectedOneOfHexadecimal {
            found: 'X',
            offset: input.len() - 1,
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
