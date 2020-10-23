//!
//! The lexical integer literal parser.
//!

#[cfg(test)]
mod tests;

pub mod error;
pub mod output;

use std::str;

use crate::token::lexeme::literal::integer::Integer;

use self::error::Error;
use self::output::Output;

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
pub fn parse(input: &str) -> Result<Output, Error> {
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
                    return Ok(Output::new(size, Integer::new_decimal(integer)));
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
                    return Ok(Output::new(size, Integer::new_binary(integer)));
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
                    return Ok(Output::new(size, Integer::new_octal(integer)));
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
                    return Ok(Output::new(size, Integer::new_decimal(integer)));
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
                    return Ok(Output::new(size - 1, Integer::new_decimal(integer)));
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

                    return Ok(Output::new(
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

                    return Ok(Output::new(
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
                    return Ok(Output::new(size, Integer::new_hexadecimal(integer)));
                }
            }
        }
    }

    match state {
        State::Start => Err(Error::UnexpectedEnd),
        State::ZeroOrNotDecimal => Ok(Output::new(size, Integer::new_decimal(integer))),
        State::Binary => {
            if !integer.is_empty() {
                Ok(Output::new(size, Integer::new_binary(integer)))
            } else {
                Err(Error::EmptyBinaryBody { offset: size })
            }
        }
        State::Octal => {
            if !integer.is_empty() {
                Ok(Output::new(size, Integer::new_octal(integer)))
            } else {
                Err(Error::EmptyOctalBody { offset: size })
            }
        }
        State::Decimal => Ok(Output::new(size, Integer::new_decimal(integer))),
        State::DecimalAfterPoint => {
            let fractional = if fractional.is_empty() {
                None
            } else {
                Some(fractional)
            };

            Ok(Output::new(
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

            Ok(Output::new(
                size,
                Integer::new_decimal_with_exponent(integer, fractional, exponent),
            ))
        }
        State::Hexadecimal => {
            if !integer.is_empty() {
                Ok(Output::new(size, Integer::new_hexadecimal(integer)))
            } else {
                Err(Error::EmptyHexadecimalBody { offset: size })
            }
        }
    }
}
