//!
//! The statement parser.
//!

mod debug;
mod r#let;
mod require;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::Error;

use self::debug::Parser as DebugParser;
use self::r#let::Parser as LetParser;
use self::require::Parser as RequireParser;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(self, mut stream: TokenStream) -> Result<(TokenStream, Statement), Error> {
        const EXPECTED: [&str; 3] = ["let", "require", "debug"];

        match stream.peek() {
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Let),
                ..
            })) => LetParser::default()
                .parse(stream)
                .map(|(stream, r#let)| (stream, Statement::Let(r#let))),
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Require),
                ..
            })) => RequireParser::default()
                .parse(stream)
                .map(|(stream, require)| (stream, Statement::Require(require))),
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Debug),
                ..
            })) => DebugParser::default()
                .parse(stream)
                .map(|(stream, debug)| (stream, Statement::Debug(debug))),
            Some(Ok(Token { lexeme, location })) => Err(Error::Syntax(SyntaxError::Expected(
                location.to_owned(),
                EXPECTED.to_vec(),
                lexeme.to_owned(),
            ))),
            Some(Err(error)) => Err(Error::Lexical(error.clone())),
            None => Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
        }
    }
}
