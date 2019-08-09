//!
//! The lexical analysis.
//!

mod alphabet;
mod comment;
mod delimiter;
mod error;
mod identifier;
mod keyword;
mod literal;
mod operator;
mod punctuation;
mod stream;
mod token;

pub use self::alphabet::Alphabet;
pub use self::comment::Comment;
pub use self::delimiter::Delimiter;
pub use self::error::Error;
pub use self::identifier::Error as IdentifierError;
pub use self::identifier::Identifier;
pub use self::keyword::Error as KeywordError;
pub use self::keyword::Keyword;
pub use self::literal::Integer as IntegerLiteral;
pub use self::literal::Literal;
pub use self::operator::Operator;
pub use self::punctuation::Punctuation;
pub use self::stream::CommentParserError;
pub use self::stream::IntegerParserError;
pub use self::stream::OperatorParserError;
pub use self::stream::TokenStream;
pub use self::token::Lexeme;
pub use self::token::Location;
pub use self::token::Token;

#[cfg(test)]
mod tests {
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
} /* This is the end of the mega ultra witness input *"#;

        let result: Vec<Token> = TokenStream::new(code.to_vec())
            .map(|result| result.unwrap())
            .collect();

        let expected: Vec<Token> = vec![
            Token {
                lexeme: Lexeme::Keyword(Keyword::Inputs),
                location: Location::new(4, 1),
            },
            Token {
                lexeme: Lexeme::Delimiter(Delimiter::BracketCurlyOpen),
                location: Location::new(4, 8),
            },
            Token {
                lexeme: Lexeme::Identifier(Identifier("a".to_string())),
                location: Location::new(5, 5),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Colon),
                location: Location::new(5, 6),
            },
            Token {
                lexeme: Lexeme::Keyword(Keyword::Uint(8)),
                location: Location::new(5, 8),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Semicolon),
                location: Location::new(5, 13),
            },
            Token {
                lexeme: Lexeme::Identifier(Identifier("b".to_string())),
                location: Location::new(6, 5),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Colon),
                location: Location::new(6, 6),
            },
            Token {
                lexeme: Lexeme::Keyword(Keyword::Field),
                location: Location::new(6, 8),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Semicolon),
                location: Location::new(6, 13),
            },
            Token {
                lexeme: Lexeme::Identifier(Identifier("c".to_string())),
                location: Location::new(7, 5),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Colon),
                location: Location::new(7, 6),
            },
            Token {
                lexeme: Lexeme::Keyword(Keyword::Bool),
                location: Location::new(7, 8),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Semicolon),
                location: Location::new(7, 12),
            },
            Token {
                lexeme: Lexeme::Delimiter(Delimiter::BracketCurlyClose),
                location: Location::new(8, 1),
            },
            Token {
                lexeme: Lexeme::Keyword(Keyword::Witness),
                location: Location::new(13, 1),
            },
            Token {
                lexeme: Lexeme::Delimiter(Delimiter::BracketCurlyOpen),
                location: Location::new(13, 9),
            },
            Token {
                lexeme: Lexeme::Identifier(Identifier("d".to_string())),
                location: Location::new(14, 5),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Colon),
                location: Location::new(14, 6),
            },
            Token {
                lexeme: Lexeme::Keyword(Keyword::Int(126)),
                location: Location::new(14, 8),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Semicolon),
                location: Location::new(14, 14),
            },
            Token {
                lexeme: Lexeme::Identifier(Identifier("e".to_string())),
                location: Location::new(15, 5),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Colon),
                location: Location::new(15, 6),
            },
            Token {
                lexeme: Lexeme::Keyword(Keyword::Field),
                location: Location::new(15, 8),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Semicolon),
                location: Location::new(15, 13),
            },
            Token {
                lexeme: Lexeme::Identifier(Identifier("f".to_string())),
                location: Location::new(16, 5),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Colon),
                location: Location::new(16, 6),
            },
            Token {
                lexeme: Lexeme::Keyword(Keyword::Bool),
                location: Location::new(16, 8),
            },
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Semicolon),
                location: Location::new(16, 12),
            },
            Token {
                lexeme: Lexeme::Delimiter(Delimiter::BracketCurlyClose),
                location: Location::new(17, 1),
            },
        ];

        assert_eq!(result, expected);
    }
}
