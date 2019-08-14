//!
//! The statement parser.
//!

mod debug;
mod r#let;
mod require;

use std::cell::RefCell;
use std::rc::Rc;

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
    pub fn parse(self, stream: Rc<RefCell<TokenStream>>) -> Result<Statement, Error> {
        const EXPECTED: [&str; 3] = ["let", "require", "debug"];

        let peek = stream.borrow_mut().peek();
        match peek {
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Let),
                ..
            })) => LetParser::default()
                .parse(stream.clone())
                .map(Statement::Let),
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Require),
                ..
            })) => RequireParser::default()
                .parse(stream.clone())
                .map(Statement::Require),
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Debug),
                ..
            })) => DebugParser::default()
                .parse(stream.clone())
                .map(Statement::Debug),
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
