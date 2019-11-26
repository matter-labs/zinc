//!
//! The lexical parser semantic.tests.
//!

#![cfg(test)]

use crate::lexical::Error;
use crate::lexical::Identifier;
use crate::lexical::IntegerLiteral;
use crate::lexical::IntegerParserError;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::SymbolParserError;
use crate::lexical::Token;
use crate::lexical::TokenStream;

static PANIC_INPUT_ENDS_WITH_EOF: &str = "The input must end with an EOF lexeme";

#[test]
fn ok() {
    let input = r#"
/*
    This is the mega ultra test application!
*/
let mut c: u8 = 2 + 2;
"#;

    let expected = vec![
        Token {
            lexeme: Lexeme::Keyword(Keyword::Let),
            location: Location::new(5, 1),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Mut),
            location: Location::new(5, 5),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("c".to_owned())),
            location: Location::new(5, 9),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(5, 10),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::new_integer_unsigned(8)),
            location: Location::new(5, 12),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Equals),
            location: Location::new(5, 15),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(IntegerLiteral::new_decimal(
                "2".to_owned(),
            ))),
            location: Location::new(5, 17),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Plus),
            location: Location::new(5, 19),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(IntegerLiteral::new_decimal(
                "2".to_owned(),
            ))),
            location: Location::new(5, 21),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(5, 22),
        },
    ]
    .into_iter()
    .collect::<Vec<Token>>();

    let mut result = Vec::with_capacity(expected.len());
    let mut stream = TokenStream::new(input.to_owned());
    loop {
        match stream.next().expect(PANIC_INPUT_ENDS_WITH_EOF) {
            Token {
                lexeme: Lexeme::Eof,
                ..
            } => break,
            token => result.push(token),
        }
    }

    assert_eq!(expected, result);
}

#[test]
fn err_unexpected_end() {
    let input = "&";

    let expected: Result<Token, Error> = Err(Error::UnexpectedEnd(Location::new(1, 1)));

    let result = TokenStream::new(input.to_owned()).next();

    assert_eq!(expected, result);
}

#[test]
fn err_unknown_character() {
    let input = "#";

    let expected: Result<Token, Error> = Err(Error::InvalidCharacter(Location::new(1, 1), '#'));

    let result = TokenStream::new(input.to_owned()).next();

    assert_eq!(expected, result);
}

#[test]
fn err_invalid_symbol() {
    let input = "|#";

    let expected: Result<Token, Error> = Err(Error::InvalidSymbol(
        Location::new(1, 1),
        SymbolParserError::InvalidCharacter('#', 2, "|#".to_owned()),
    ));

    let result = TokenStream::new(input.to_owned()).next();

    assert_eq!(expected, result);
}

#[test]
fn err_invalid_integer_literal() {
    let input = "0xCRAP";

    let expected: Result<Token, Error> = Err(Error::InvalidIntegerLiteral(
        Location::new(1, 1),
        IntegerParserError::InvalidHexadecimalCharacter('R', 4, "0xCR".to_owned()),
    ));

    let result = TokenStream::new(input.to_owned()).next();

    assert_eq!(expected, result);
}
