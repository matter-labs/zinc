//!
//! The lexical tests.
//!

#![cfg(test)]

use super::*;

#[test]
fn ok() {
    let code = br#"
/*
    This is the mega ultra test application!
*/
inputs {
    a: uint1; // input 1
}

witness {
    b: int253; /* witness 1 */
}

let mut c: uint228 = 2 + 2;
"#;

    let expected: Vec<Token> = vec![
        Token {
            lexeme: Lexeme::Keyword(Keyword::Inputs),
            location: Location::new(5, 1),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
            location: Location::new(5, 8),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier::new(b"a")),
            location: Location::new(6, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(6, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::uint(1)),
            location: Location::new(6, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(6, 13),
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
            lexeme: Lexeme::Identifier(Identifier::new(b"b")),
            location: Location::new(10, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(10, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::int(253)),
            location: Location::new(10, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(10, 14),
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
            lexeme: Lexeme::Identifier(Identifier::new(b"c")),
            location: Location::new(13, 9),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(13, 10),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::uint(228)),
            location: Location::new(13, 12),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Equals),
            location: Location::new(13, 20),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"2".to_vec()))),
            location: Location::new(13, 22),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Plus),
            location: Location::new(13, 24),
        },
        Token {
            lexeme: Lexeme::Literal(Literal::Integer(IntegerLiteral::decimal(b"2".to_vec()))),
            location: Location::new(13, 26),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(13, 27),
        },
    ];

    let result: Vec<Token> = TokenStream::new(code.to_vec())
        .map(|result| result.expect("Lexical error"))
        .collect();

    assert_eq!(expected, result);
}
