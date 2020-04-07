//!
//! The module-local statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::keyword::Keyword;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::statement::module::Parser as ModStatementParser;
use crate::syntax::parser::statement::r#const::Parser as ConstStatementParser;
use crate::syntax::parser::statement::r#enum::Parser as EnumStatementParser;
use crate::syntax::parser::statement::r#fn::Parser as FnStatementParser;
use crate::syntax::parser::statement::r#impl::Parser as ImplStatementParser;
use crate::syntax::parser::statement::r#struct::Parser as StructStatementParser;
use crate::syntax::parser::statement::r#type::Parser as TypeStatementParser;
use crate::syntax::parser::statement::r#use::Parser as UseStatementParser;
use crate::syntax::tree::statement::local_mod::Statement as ModuleLocalStatement;

static HINT_ONLY_SOME_STATEMENTS: &str =
    "only constants, types, functions, and type implementations may be declared at the module root";

#[derive(Default)]
pub struct Parser {}

impl Parser {
    ///
    /// Parses a top-level statement allowed in modules.
    ///
    pub fn parse(
        self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ModuleLocalStatement, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Const),
                ..
            } => ConstStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Const(statement), next)),
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Type),
                ..
            } => TypeStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Type(statement), next)),
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Struct),
                ..
            } => StructStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Struct(statement), next)),
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Enum),
                ..
            } => EnumStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Enum(statement), next)),
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Fn),
                ..
            } => FnStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Fn(statement), next)),
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Mod),
                ..
            } => ModStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Mod(statement), next)),
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Use),
                ..
            } => UseStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Use(statement), next)),
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Impl),
                ..
            } => ImplStatementParser::default()
                .parse(stream, Some(token))
                .map(|(statement, next)| (ModuleLocalStatement::Impl(statement), next)),
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                location,
            } => Ok((ModuleLocalStatement::Empty(location), None)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::expected_one_of(
                location,
                vec![
                    "type", "struct", "enum", "fn", "mod", "use", "impl", "const",
                ],
                lexeme,
                Some(HINT_ONLY_SOME_STATEMENTS),
            ))),
        }
    }
}
