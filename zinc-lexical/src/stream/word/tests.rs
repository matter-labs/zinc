//!
//! The lexical word parser tests.
//!

use super::parse;
use super::Output;
use crate::token::lexeme::identifier::Identifier;
use crate::token::lexeme::keyword::Keyword;
use crate::token::lexeme::literal::boolean::Boolean;
use crate::token::lexeme::literal::Literal;
use crate::token::lexeme::symbol::Symbol;
use crate::token::lexeme::Lexeme;

#[test]
fn ok_identifier() {
    let input = "xyz";
    let expected = Output::new(
        input.len(),
        Lexeme::Identifier(Identifier::new(input.to_owned())),
    );
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_identifier_below_field_range() {
    let input = "u0";
    let expected = Output::new(
        input.len(),
        Lexeme::Identifier(Identifier::new(input.to_owned())),
    );
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_identifier_above_field_range() {
    let input = "u256";
    let expected = Output::new(
        input.len(),
        Lexeme::Identifier(Identifier::new(input.to_owned())),
    );
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_identifier_invalid_modulo() {
    let input = "u119";
    let expected = Output::new(
        input.len(),
        Lexeme::Identifier(Identifier::new(input.to_owned())),
    );
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_keyword() {
    let input = "match";
    let expected = Output::new(input.len(), Lexeme::Keyword(Keyword::Match));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_keyword_signed_integer_min() {
    let input = "i8";
    let expected = Output::new(input.len(), Lexeme::Keyword(Keyword::new_integer_signed(8)));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_keyword_unsigned_integer_max() {
    let input = "u248";
    let expected = Output::new(
        input.len(),
        Lexeme::Keyword(Keyword::new_integer_unsigned(
            zinc_const::bitlength::INTEGER_MAX,
        )),
    );
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_literal_boolean() {
    let input = "true";
    let expected = Output::new(
        input.len(),
        Lexeme::Literal(Literal::Boolean(Boolean::r#true())),
    );
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_symbol_underscore() {
    let input = "_";
    let expected = Output::new(input.len(), Lexeme::Symbol(Symbol::Underscore));
    let result = parse(input);
    assert_eq!(result, expected);
}
