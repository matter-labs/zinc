//!
//! The lexical parser semantic.tests.
//!

#![cfg(test)]

use crate::lexical::error::Error;
use crate::lexical::stream::integer::Error as IntegerParserError;
use crate::lexical::stream::symbol::Error as SymbolParserError;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::identifier::Identifier;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::literal::integer::Integer;
use crate::lexical::token::lexeme::literal::Literal;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::location::Location;
use crate::lexical::token::Token;

static PANIC_LEXICAL_ERROR: &str = "An unexpected lexical error";

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
            lexeme: Lexeme::Literal(Literal::Integer(Integer::new_decimal("2".to_owned()))),
            location: Location::new(5, 17),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Plus),
            location: Location::new(5, 19),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(Integer::new_decimal("2".to_owned()))),
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
        match stream.next().expect(PANIC_LEXICAL_ERROR) {
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

    let expected: Result<Token, Error> = Err(Error::InvalidSymbol(
        Location::new(1, 1),
        SymbolParserError::InvalidCharacter('#', 1, "#".to_owned()),
    ));

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

    let expected: Result<Token, Error> = Err(Error::InvalidInteger(
        Location::new(1, 1),
        IntegerParserError::InvalidHexadecimalCharacter('R', 4, "0xCR".to_owned()),
    ));

    let result = TokenStream::new(input.to_owned()).next();

    assert_eq!(expected, result);
}
