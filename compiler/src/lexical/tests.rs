//!
//! The lexical tests.
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

#[test]
fn ok() {
    let input = r#"
/*
    This is the mega ultra test application!
*/
inputs {
    a: u8, // input 1
}

witness {
    b: i248, /* witness 1 */
}

let mut c: u232 = 2 + 2;
"#;

    let expected = vec![
        Token {
            lexeme: Lexeme::Keyword(Keyword::Inputs),
            location: Location::new(5, 1),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
            location: Location::new(5, 8),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("a".to_owned())),
            location: Location::new(6, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(6, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::new_integer_unsigned(8)),
            location: Location::new(6, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Comma),
            location: Location::new(6, 10),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
            location: Location::new(7, 1),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Witness),
            location: Location::new(9, 1),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
            location: Location::new(9, 9),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("b".to_owned())),
            location: Location::new(10, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(10, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::new_integer_signed(248)),
            location: Location::new(10, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Comma),
            location: Location::new(10, 12),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
            location: Location::new(11, 1),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Let),
            location: Location::new(13, 1),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Mut),
            location: Location::new(13, 5),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("c".to_owned())),
            location: Location::new(13, 9),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(13, 10),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::new_integer_unsigned(232)),
            location: Location::new(13, 12),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Equals),
            location: Location::new(13, 17),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(IntegerLiteral::new_decimal(
                "2".to_owned(),
            ))),
            location: Location::new(13, 19),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Plus),
            location: Location::new(13, 21),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(IntegerLiteral::new_decimal(
                "2".to_owned(),
            ))),
            location: Location::new(13, 23),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(13, 24),
        },
    ]
    .into_iter()
    .map(Result::Ok)
    .collect::<Vec<Result<Token, Error>>>();

    let result = TokenStream::new(input.to_owned())
        .into_iter()
        .collect::<Vec<Result<Token, Error>>>();

    assert_eq!(expected, result);
}

#[test]
fn err_unexpected_end() {
    let input = "&";

    let expected: Result<Token, Error> = Err(Error::UnexpectedEnd(Location::new(1, 1)));

    let result = TokenStream::new(input.to_owned())
        .next()
        .expect("Always contains an element");

    assert_eq!(expected, result);
}

#[test]
fn err_unknown_character() {
    let input = "#";

    let expected: Result<Token, Error> = Err(Error::InvalidCharacter(Location::new(1, 1), '#'));

    let result = TokenStream::new(input.to_owned())
        .next()
        .expect("Always contains an element");

    assert_eq!(expected, result);
}

#[test]
fn err_invalid_symbol() {
    let input = "|#";

    let expected: Result<Token, Error> = Err(Error::InvalidSymbol(
        Location::new(1, 1),
        SymbolParserError::InvalidCharacter('#', 2, "|#".to_owned()),
    ));

    let result = TokenStream::new(input.to_owned())
        .next()
        .expect("Always contains an element");

    assert_eq!(expected, result);
}

#[test]
fn err_invalid_integer_literal() {
    let input = "0xCRAP";

    let expected: Result<Token, Error> = Err(Error::InvalidIntegerLiteral(
        Location::new(1, 1),
        IntegerParserError::InvalidHexadecimalCharacter('R', 4, "0xCR".to_owned()),
    ));

    let result = TokenStream::new(input.to_owned())
        .next()
        .expect("Always contains an element");

    assert_eq!(expected, result);
}
