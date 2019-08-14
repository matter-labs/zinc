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
} /* This is the end of the mega ultra witness input */"#;

    let result: Vec<Token> = TokenStream::new(code.to_vec())
        .map(|result| result.expect("Token error"))
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
            lexeme: Lexeme::Identifier(Identifier("a".to_string())),
            location: Location::new(5, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(5, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Uint(8)),
            location: Location::new(5, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(5, 13),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier("b".to_string())),
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
            lexeme: Lexeme::Identifier(Identifier("c".to_string())),
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
            lexeme: Lexeme::Identifier(Identifier("d".to_string())),
            location: Location::new(14, 5),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Colon),
            location: Location::new(14, 6),
        },
        Token {
            lexeme: Lexeme::Keyword(Keyword::Int(126)),
            location: Location::new(14, 8),
        },
        Token {
            lexeme: Lexeme::Symbol(Symbol::Semicolon),
            location: Location::new(14, 14),
        },
        Token {
            lexeme: Lexeme::Identifier(Identifier("e".to_string())),
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
            lexeme: Lexeme::Identifier(Identifier("f".to_string())),
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
    ];

    assert_eq!(result, expected);
}
