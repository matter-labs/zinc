//!
//! The lexical integer literal parser tests.
//!

use super::parse;
use super::Error;
use super::Output;
use crate::token::lexeme::literal::integer::Integer;

#[test]
fn ok_binary() {
    let input = "0b101010";
    let filtered = "101010";
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_binary(filtered.to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_octal() {
    let input = "0o42";
    let filtered = "42";
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_octal(filtered.to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_decimal_zero() {
    let input = "0";
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_decimal(input.to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_decimal_zero_with_fractional() {
    let input = "0.42";
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_decimal_with_exponent("0".to_owned(), Some("42".to_owned()), None),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_decimal_zero_with_fractional_and_exponent() {
    let input = "0.42E2";
    let expected = Ok(Output::new(
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
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_decimal(input.to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_decimal_with_fractional() {
    let input = "666.42";
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_decimal_with_exponent("666".to_owned(), Some("42".to_owned()), None),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_decimal_with_fractional_and_exponent() {
    let input = "666.42E2";
    let expected = Ok(Output::new(
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
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_hexadecimal(filtered.to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_hexadecimal_uppercase() {
    let input = "0xDEAD_666_BEEF";
    let filtered = "dead666beef";
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_hexadecimal(filtered.to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_hexadecimal_mixed_case() {
    let input = "0xdEaD_666_bEeF";
    let filtered = "dead666beef";
    let expected = Ok(Output::new(
        input.len(),
        Integer::new_hexadecimal(filtered.to_owned()),
    ));
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
