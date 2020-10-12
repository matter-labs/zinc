//!
//! The lexical integer literal parser.
//!

use std::str;

use crate::token::lexeme::literal::integer::Integer;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    Start,
    /// The `0` has been parsed so far.
    ZeroOrNotDecimal,
    /// The `0b` has been parsed so far.
    Binary,
    /// The `0o` has been parsed so far.
    Octal,
    /// The non-zero decimal character has been parsed so far.
    Decimal,
    /// Some decimal characters and the point have been parsed so far.
    DecimalAfterPoint,
    /// Some decimal characters and the exponent symbol have been parsed so far.
    DecimalAfterExponent,
    /// The `0x` has been parsed so far.
    Hexadecimal,
}

///
/// The parser error.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// The lexeme is not an integer, which means that another parser must be run.
    NotAnInteger,
    /// The lexeme is `0b`, which is not a valid binary literal.
    EmptyBinaryBody {
        /// The position where the literal ends.
        offset: usize,
    },
    /// The lexeme is `0o`, which is not a valid octal literal.
    EmptyOctalBody {
        /// The position where the literal ends.
        offset: usize,
    },
    /// The decimal literal exponent cannot be empty.
    EmptyExponent {
        /// The position of the invalid character.
        offset: usize,
    },
    /// The lexeme is `0x`, which is not a valid hexadecimal literal.
    EmptyHexadecimalBody {
        /// The position where the literal ends.
        offset: usize,
    },
    /// A non-binary character is found in a binary literal.
    ExpectedOneOfBinary {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// A non-octal character is found in an octal literal.
    ExpectedOneOfOctal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// A non-decimal character is found in a decimal literal.
    ExpectedOneOfDecimal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// A non-hexadecimal character is found in a hexadecimal literal.
    ExpectedOneOfHexadecimal {
        /// The invalid character.
        found: char,
        /// The position of the invalid character.
        offset: usize,
    },
    /// Unable to finish a literal.
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
/// '42', '.05E18', '0.01E9', '15E18'
///
/// 4. Hexadecimal
/// '2a'
///
pub fn parse(input: &str) -> Result<(usize, Integer), Error> {
    let mut state = State::Start;
    let mut size = 0;

    let mut integer = String::with_capacity(40);
    let mut fractional = String::with_capacity(40);
    let mut exponent = String::with_capacity(2);

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::Start => {
                if character == Integer::CHARACTER_ZERO {
                    integer.push(character);
                    size += 1;
                    state = State::ZeroOrNotDecimal;
                } else if character == Integer::CHARACTER_DECIMAL_POINT {
                    size += 1;
                    state = State::DecimalAfterPoint;
                } else if Integer::CHARACTERS_DECIMAL.contains(&character) {
                    integer.push(character);
                    size += 1;
                    state = State::Decimal;
                } else {
                    return Err(Error::NotAnInteger);
                }
            }
            State::ZeroOrNotDecimal => {
                if character == Integer::CHARACTER_INITIAL_BINARY {
                    size += 1;
                    integer.clear();
                    state = State::Binary;
                } else if character == Integer::CHARACTER_INITIAL_OCTAL {
                    size += 1;
                    integer.clear();
                    state = State::Octal;
                } else if character == Integer::CHARACTER_INITIAL_HEXADECIMAL {
                    size += 1;
                    integer.clear();
                    state = State::Hexadecimal;
                } else if character == Integer::CHARACTER_DECIMAL_POINT {
                    size += 1;
                    state = State::DecimalAfterPoint;
                } else if character.is_ascii_alphanumeric() {
                    return Err(Error::ExpectedOneOfDecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_decimal(integer)));
                }
            }
            State::Binary => {
                if Integer::CHARACTERS_BINARY.contains(&character) {
                    integer.push(character.to_ascii_lowercase());
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character.is_ascii_alphanumeric() || size <= 2 {
                    return Err(Error::ExpectedOneOfBinary {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_binary(integer)));
                }
            }
            State::Octal => {
                if Integer::CHARACTERS_OCTAL.contains(&character) {
                    integer.push(character.to_ascii_lowercase());
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character.is_ascii_alphanumeric() || size <= 2 {
                    return Err(Error::ExpectedOneOfOctal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_octal(integer)));
                }
            }
            State::Decimal => {
                if Integer::CHARACTERS_DECIMAL.contains(&character) {
                    integer.push(character);
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character == Integer::CHARACTER_DECIMAL_POINT {
                    size += 1;
                    state = State::DecimalAfterPoint;
                } else if character == Integer::CHARACTER_EXPONENT {
                    size += 1;
                    state = State::DecimalAfterExponent;
                } else if character.is_ascii_alphanumeric() {
                    return Err(Error::ExpectedOneOfDecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_decimal(integer)));
                }
            }
            State::DecimalAfterPoint => {
                if Integer::CHARACTERS_DECIMAL.contains(&character) {
                    fractional.push(character);
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character == Integer::CHARACTER_DECIMAL_POINT {
                    // encountered a range operator, go one symbol back and return
                    return Ok((size - 1, Integer::new_decimal(integer)));
                } else if character == Integer::CHARACTER_EXPONENT {
                    size += 1;
                    state = State::DecimalAfterExponent;
                } else if character.is_ascii_alphanumeric() {
                    return Err(Error::ExpectedOneOfDecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    let fractional = if fractional.is_empty() {
                        None
                    } else {
                        Some(fractional)
                    };

                    return Ok((
                        size,
                        Integer::new_decimal_with_exponent(integer, fractional, None),
                    ));
                }
            }
            State::DecimalAfterExponent => {
                if Integer::CHARACTERS_DECIMAL.contains(&character) {
                    exponent.push(character);
                    size += 1;
                } else if character.is_ascii_alphanumeric() {
                    return Err(Error::ExpectedOneOfDecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    let fractional = if fractional.is_empty() {
                        None
                    } else {
                        Some(fractional)
                    };

                    let exponent = if exponent.is_empty() {
                        return Err(Error::EmptyExponent { offset: size });
                    } else {
                        Some(exponent)
                    };

                    return Ok((
                        size,
                        Integer::new_decimal_with_exponent(integer, fractional, exponent),
                    ));
                }
            }
            State::Hexadecimal => {
                if Integer::CHARACTERS_HEXADECIMAL.contains(&character) {
                    integer.push(character.to_ascii_lowercase());
                    size += 1;
                } else if character == Integer::CHARACTER_DELIMITER {
                    size += 1;
                } else if character.is_ascii_alphanumeric() || size <= 2 {
                    return Err(Error::ExpectedOneOfHexadecimal {
                        found: character,
                        offset: size,
                    });
                } else {
                    return Ok((size, Integer::new_hexadecimal(integer)));
                }
            }
        }
    }

    match state {
        State::Start => Err(Error::UnexpectedEnd),
        State::ZeroOrNotDecimal => Ok((size, Integer::new_decimal(integer))),
        State::Binary => {
            if !integer.is_empty() {
                Ok((size, Integer::new_binary(integer)))
            } else {
                Err(Error::EmptyBinaryBody { offset: size })
            }
        }
        State::Octal => {
            if !integer.is_empty() {
                Ok((size, Integer::new_octal(integer)))
            } else {
                Err(Error::EmptyOctalBody { offset: size })
            }
        }
        State::Decimal => Ok((size, Integer::new_decimal(integer))),
        State::DecimalAfterPoint => {
            let fractional = if fractional.is_empty() {
                None
            } else {
                Some(fractional)
            };

            Ok((
                size,
                Integer::new_decimal_with_exponent(integer, fractional, None),
            ))
        }
        State::DecimalAfterExponent => {
            let fractional = if fractional.is_empty() {
                None
            } else {
                Some(fractional)
            };

            let exponent = if exponent.is_empty() {
                return Err(Error::EmptyExponent { offset: size });
            } else {
                Some(exponent)
            };

            Ok((
                size,
                Integer::new_decimal_with_exponent(integer, fractional, exponent),
            ))
        }
        State::Hexadecimal => {
            if !integer.is_empty() {
                Ok((size, Integer::new_hexadecimal(integer)))
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
    use crate::token::lexeme::literal::integer::Integer;

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
    fn ok_decimal_zero_with_fractional() {
        let input = "0.42";
        let expected = Ok((
            input.len(),
            Integer::new_decimal_with_exponent("0".to_owned(), Some("42".to_owned()), None),
        ));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_decimal_zero_with_fractional_and_exponent() {
        let input = "0.42E2";
        let expected = Ok((
            input.len(),
            Integer::new_decimal_with_exponent(
                "0".to_owned(),
                Some("42".to_owned()),
                Some("2".to_owned()),
            ),
        ));
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
    fn ok_decimal_with_fractional() {
        let input = "666.42";
        let expected = Ok((
            input.len(),
            Integer::new_decimal_with_exponent("666".to_owned(), Some("42".to_owned()), None),
        ));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_decimal_with_fractional_and_exponent() {
        let input = "666.42E2";
        let expected = Ok((
            input.len(),
            Integer::new_decimal_with_exponent(
                "666".to_owned(),
                Some("42".to_owned()),
                Some("2".to_owned()),
            ),
        ));
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
    fn error_empty_exponent() {
        let input = "42.5E";
        let expected = Err(Error::EmptyExponent {
            offset: input.len(),
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_unexpected_exponent() {
        let input = "";
        let expected = Err(Error::UnexpectedEnd);
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
    fn error_expected_one_of_decimal_zero_with_exponent() {
        let input = "0E";
        let expected = Err(Error::ExpectedOneOfDecimal {
            found: Integer::CHARACTER_EXPONENT,
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
