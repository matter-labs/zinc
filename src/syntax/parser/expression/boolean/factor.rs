//!
//! The boolean factor syntax parser.
//!

use log::*;

use crate::lexical::BooleanLiteral;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::Error;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(
        mut self,
        mut iterator: TokenStream,
    ) -> Result<(TokenStream, BooleanLiteral), Error> {
        match iterator.next() {
            Some(Ok(Token {
                lexeme: Lexeme::Literal(Literal::Boolean(boolean)),
                ..
            })) => Ok((iterator, boolean)),
            _ => unimplemented!(),
        }
    }
}
