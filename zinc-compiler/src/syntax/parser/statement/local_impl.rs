//!
//! The implementation-local statement parser.
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
use crate::syntax::parser::statement::r#const::Parser as ConstStatementParser;
use crate::syntax::parser::statement::r#fn::Parser as FnStatementParser;
use crate::syntax::tree::statement::local_impl::Statement as ImplementationLocalStatement;

static HINT_ONLY_SOME_STATEMENTS: &str =
    "only constants and functions may be declared within a type implementation";

#[derive(Default)]
pub struct Parser {}

impl Parser {
    ///
    /// Parses a statement allowed in type implementations.
    ///
    pub fn parse(
        self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ImplementationLocalStatement, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
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
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                location,
            } => Ok((ImplementationLocalStatement::Empty(location), None)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::expected_one_of(
                location,
                vec!["const", "fn"],
                lexeme,
                Some(HINT_ONLY_SOME_STATEMENTS),
            ))),
        }
    }
}
