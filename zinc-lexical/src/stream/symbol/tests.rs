//!
//! The lexical symbol parser tests.
//!

use super::parse;
use super::Error;
use super::Output;
use crate::token::lexeme::symbol::Symbol;

#[test]
fn ok() {
    let input = "==";
    let expected = Ok(Output::new(input.len(), Symbol::DoubleEquals));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn error_invalid_character() {
    let input = "@";
    let expected = Err(Error::InvalidCharacter {
        found: input.chars().collect::<Vec<char>>()[0],
        offset: 0,
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
