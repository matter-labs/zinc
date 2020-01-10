//!
//! The implementation-local statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ConstStatementParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::FnStatementParser;
use crate::syntax::ImplementationLocalStatement;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(
        self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ImplementationLocalStatement, Option<Token>), Error> {
        match match initial.take() {
            Some(token) => token,
            None => stream.borrow_mut().next()?,
        } {
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Const),
                ..
            } => ConstStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ImplementationLocalStatement::Const(statement), next)),
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Fn),
                ..
            } => FnStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ImplementationLocalStatement::Fn(statement), next)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec!["const", "fn"],
                lexeme,
            ))),
        }
    }
}
