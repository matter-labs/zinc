//!
//! The syntax parser of expression.
//!

mod boolean;

use std::collections::LinkedList;

use log::*;

use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::Error;

use self::boolean::Parser as BooleanParser;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(
        mut self,
        mut iterator: TokenStream,
    ) -> Result<(TokenStream, LinkedList<Lexeme>), Error> {
        BooleanParser::default().parse(iterator)
    }
}
