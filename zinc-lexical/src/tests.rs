//!
//! The lexical parser tests.
//!

use crate::error::Error;
use crate::stream::TokenStream;
use crate::token::lexeme::identifier::Identifier;
use crate::token::lexeme::keyword::Keyword;
use crate::token::lexeme::literal::integer::Integer;
use crate::token::lexeme::literal::Literal;
use crate::token::lexeme::symbol::Symbol;
use crate::token::lexeme::Lexeme;
use crate::token::location::Location;
use crate::token::Token;

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
            location: Location::test(5, 1),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Mut),
            location: Location::test(5, 5),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("c".to_owned())),
            location: Location::test(5, 9),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::test(5, 10),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::new_integer_unsigned(8)),
            location: Location::test(5, 12),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Equals),
            location: Location::test(5, 15),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(Integer::new_decimal("2".to_owned()))),
            location: Location::test(5, 17),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Plus),
            location: Location::test(5, 19),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(Integer::new_decimal("2".to_owned()))),
            location: Location::test(5, 21),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::test(5, 22),
        },
    ]
    .into_iter()
    .collect::<Vec<Token>>();

    let mut result = Vec::with_capacity(expected.len());
    let mut stream = TokenStream::test(input);
    loop {
        match stream.next().expect(zinc_const::panic::TEST_DATA_VALID) {
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
        Location::test(1, 1),
        Location::test(1, 16),
    ));

    let result = TokenStream::test(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_unterminated_double_quote_string() {
    let input = "\"double quote string";

    let expected: Result<Token, Error> = Err(Error::unterminated_double_quote_string(
        Location::test(1, 1),
        Location::test(1, 21),
    ));

    let result = TokenStream::test(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_expected_one_of_binary() {
    let input = "0b102";

    let expected: Result<Token, Error> =
        Err(Error::expected_one_of_binary(Location::test(1, 5), '2'));

    let result = TokenStream::test(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_expected_one_of_octal() {
    let input = "0o378";

    let expected: Result<Token, Error> =
        Err(Error::expected_one_of_octal(Location::test(1, 5), '8'));

    let result = TokenStream::test(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_expected_one_of_decimal() {
    let input = "42x";

    let expected: Result<Token, Error> =
        Err(Error::expected_one_of_decimal(Location::test(1, 3), 'x'));

    let result = TokenStream::test(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_expected_one_of_hexadecimal() {
    let input = "0x42t";

    let expected: Result<Token, Error> = Err(Error::expected_one_of_hexadecimal(
        Location::test(1, 5),
        't',
    ));

    let result = TokenStream::test(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_invalid_character() {
    let input = "@";

    let expected: Result<Token, Error> = Err(Error::invalid_character(
        Location::test(1, 1),
        input.chars().collect::<Vec<char>>()[0],
    ));

    let result = TokenStream::test(input).next();

    assert_eq!(result, expected);
}

#[test]
fn error_unexpected_end() {
    let input = "0x";

    let expected: Result<Token, Error> = Err(Error::unexpected_end(Location::test(1, 3)));

    let result = TokenStream::test(input).next();

    assert_eq!(result, expected);
}
