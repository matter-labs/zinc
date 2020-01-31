//!
//! The module-local statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ConstStatementParser;
use crate::syntax::EnumStatementParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::FnStatementParser;
use crate::syntax::ImplStatementParser;
use crate::syntax::ModStatementParser;
use crate::syntax::ModuleLocalStatement;
use crate::syntax::StaticStatementParser;
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
    ) -> Result<(ModuleLocalStatement, Option<Token>), Error> {
        match crate::syntax::take_or_next(initial.take(), stream.clone())? {
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Const),
                ..
            } => ConstStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Const(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Static),
                ..
            } => StaticStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Static(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Type),
                ..
            } => TypeStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Type(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Struct),
                ..
            } => StructStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Struct(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Enum),
                ..
            } => EnumStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Enum(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Fn),
                ..
            } => FnStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Fn(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Mod),
                ..
            } => ModStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Mod(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Use),
                ..
            } => UseStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Use(statement), next)),
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Impl),
                ..
            } => ImplStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Impl(statement), next)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec![
                    "const", "static", "type", "struct", "enum", "fn", "mod", "use", "impl",
                ],
                lexeme,
            ))),
        }
    }
}
