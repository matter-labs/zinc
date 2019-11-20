//!
//! The statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::EnumStatementParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::FnStatementParser;
use crate::syntax::ModStatementParser;
use crate::syntax::OuterStatement;
use crate::syntax::StructStatementParser;
use crate::syntax::TypeStatementParser;
use crate::syntax::UseStatementParser;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(
        self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(OuterStatement, Option<Token>), Error> {
        match match initial.take() {
            Some(token) => token,
            None => stream.borrow_mut().next()?,
        } {
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Type),
                ..
            } => TypeStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|statement| (OuterStatement::Type(statement), None)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Struct),
                ..
            } => StructStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Struct(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Enum),
                ..
            } => EnumStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Enum(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Fn),
                ..
            } => FnStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|statement| (OuterStatement::Fn(statement), None)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Mod),
                ..
            } => ModStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|statement| (OuterStatement::Mod(statement), None)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Use),
                ..
            } => UseStatementParser::default()
                .parse(stream.clone(), Some(token))
                .map(|(statement, next)| (OuterStatement::Use(statement), next)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec!["type", "struct", "enum", "fn", "mod", "use"],
                lexeme,
            ))),
        }
    }
}
