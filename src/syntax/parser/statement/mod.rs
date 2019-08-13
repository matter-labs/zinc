//!
//! The statement syntax parser.
//!

mod r#let;
mod require;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::Error;

use self::r#let::Parser as LetParser;
use self::require::Parser as RequireParser;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(self, mut iterator: TokenStream) -> Result<(TokenStream, Statement), Error> {
        const EXPECTED: [&str; 2] = ["let", "require"];

        match iterator.peek() {
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Let),
                ..
            })) => unimplemented!(),
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Require),
                ..
            })) => RequireParser::default()
                .parse(iterator)
                .map(|(iterator, require)| (iterator, Statement::Require(require))),
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
