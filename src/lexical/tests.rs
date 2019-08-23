//!
//! The lexical tests.
//!

#![cfg(test)]

use super::*;

#[test]
fn success() {
    let code = br#"/*
    This is the mega ultra inputs input!
*/
inputs {
    a: uint8; // input 1
    b: field; // input 2
    c: bool; // input 3
} /* This is the end of the mega ultra inputs input */

/*
    This is the mega ultra witness input!
*/
witness {
    d: int126; // witness 1
    e: field; // witness 2
    f: bool; // witness 3
} /* This is the end of the mega ultra witness input */

let mut x: uint228 = 2 + 2;"#;

    let result: Vec<Token> = TokenStream::new(code.to_vec())
        .map(|result| result.expect("Lexical error"))
        .collect();

    let expected: Vec<Token> = vec![
        Token {
            lexeme: Lexeme::Keyword(Keyword::Inputs),
            location: Location::new(4, 1),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
            location: Location::new(4, 8),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("a")),
            location: Location::new(5, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(5, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::uint(8)),
            location: Location::new(5, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(5, 13),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("b")),
            location: Location::new(6, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(6, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Field),
            location: Location::new(6, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(6, 13),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("c")),
            location: Location::new(7, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(7, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Bool),
            location: Location::new(7, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(7, 12),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
            location: Location::new(8, 1),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Witness),
            location: Location::new(13, 1),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
            location: Location::new(13, 9),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("d")),
            location: Location::new(14, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(14, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::int(126)),
            location: Location::new(14, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(14, 14),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("e")),
            location: Location::new(15, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(15, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Field),
            location: Location::new(15, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(15, 13),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("f")),
            location: Location::new(16, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(16, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Bool),
            location: Location::new(16, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(16, 12),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
            location: Location::new(17, 1),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Let),
            location: Location::new(19, 1),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Mut),
            location: Location::new(19, 5),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new("x")),
            location: Location::new(19, 9),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(19, 10),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::uint(228)),
            location: Location::new(19, 12),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Equals),
            location: Location::new(19, 20),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"2".to_vec()))),
            location: Location::new(19, 22),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Plus),
            location: Location::new(19, 24),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"2".to_vec()))),
            location: Location::new(19, 26),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(19, 27),
        },
    ];

    assert_eq!(result, expected);
}
