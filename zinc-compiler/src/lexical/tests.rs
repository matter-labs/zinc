//!
//! The lexical parser tests.
//!

#![cfg(test)]

use crate::lexical::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::identifier::Identifier;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::literal::integer::Integer;
use crate::lexical::token::lexeme::literal::Literal;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::location::Location;
use crate::lexical::token::Token;

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
    let mut stream = TokenStream::new(input);
    loop {
        match stream.next().expect(crate::panic::TEST_DATA_VALID) {
            Token {
                lexeme: Lexeme::Eof,
                ..
            } => break,
            token => result.push(token),
        }
    }

    assert_eq!(result, expected);
}

#[test]
fn error_unterminated_block_comment() {
    let input = "/*block comment";

    let expected: Result<Token, Error> = Err(Error::unterminated_block_comment(
        Location::new(1, 1),
        Location::new(1, 16),
    ));

    let result = TokenStream::new(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_unterminated_double_quote_string() {
    let input = "\"double quote string";

    let expected: Result<Token, Error> = Err(Error::unterminated_double_quote_string(
        Location::new(1, 1),
        Location::new(1, 21),
    ));

    let result = TokenStream::new(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_expected_one_of_binary() {
    let input = "0b102";

    let expected: Result<Token, Error> =
        Err(Error::expected_one_of_binary(Location::new(1, 5), '2'));

    let result = TokenStream::new(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_expected_one_of_octal() {
    let input = "0o378";

    let expected: Result<Token, Error> =
        Err(Error::expected_one_of_octal(Location::new(1, 5), '8'));

    let result = TokenStream::new(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_expected_one_of_decimal() {
    let input = "42x";

    let expected: Result<Token, Error> =
        Err(Error::expected_one_of_decimal(Location::new(1, 3), 'x'));

    let result = TokenStream::new(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_expected_one_of_hexadecimal() {
    let input = "0x42t";

    let expected: Result<Token, Error> =
        Err(Error::expected_one_of_hexadecimal(Location::new(1, 5), 't'));

    let result = TokenStream::new(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_invalid_character() {
    let input = "#";

    let expected: Result<Token, Error> = Err(Error::invalid_character(Location::new(1, 1), '#'));

    let result = TokenStream::new(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_unexpected_end() {
    let input = "0x";

    let expected: Result<Token, Error> = Err(Error::unexpected_end(Location::new(1, 3)));

    let result = TokenStream::new(input).next();

    assert_eq!(result, expected);
}
