//!
//! The syntax parser.
//!

pub mod expression;
pub mod field;
pub mod field_list;
pub mod pattern_binding;
pub mod pattern_binding_list;
pub mod pattern_match;
pub mod statement;
pub mod r#type;
pub mod variant;
pub mod variant_list;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::parser::statement::local_mod::Parser as ModLocalStatementParser;
use crate::syntax::tree::Tree;

#[derive(Default)]
pub struct Parser {
    next: Option<Token>,
}

impl Parser {
    ///
    /// The top-level parser. Parses a list of module level statements.
    ///
    pub fn parse(mut self, input: &str, file: Option<usize>) -> Result<Tree, Error> {
        let stream = match file {
            Some(file) => TokenStream::new_with_file(input, file),
            None => TokenStream::new(input),
        };
        let stream = Rc::new(RefCell::new(stream));

        let mut statements = Vec::new();
        loop {
            match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                Token {
                    lexeme: Lexeme::Eof,
                    ..
                } => break,
                token => {
                    let (statement, next) =
                        ModLocalStatementParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    statements.push(statement);
                }
            }
        }

        Ok(Tree { statements })
    }
}

pub fn take_or_next(
    mut token: Option<Token>,
    stream: Rc<RefCell<TokenStream>>,
) -> Result<Token, Error> {
    match token.take() {
        Some(token) => Ok(token),
        None => Ok(stream.borrow_mut().next()?),
    }
}
