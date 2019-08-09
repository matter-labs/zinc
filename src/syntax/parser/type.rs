//!
//! The syntax parser of type.
//!

use log::*;

use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Keyword,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Keyword
    }
}

#[derive(Default)]
pub struct TypeParser {
    state: State,
    builder: TypeBuilder,
}

impl TypeParser {
    pub fn parse(mut self, mut iterator: TokenStream) -> Result<(TokenStream, Type), Error> {
        loop {
            match self.state {
                State::Keyword => match iterator.next() {
                    Some(Ok(token)) => self.keyword(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => return Ok((iterator, self.builder.finish())),
            }
        }
    }

    fn keyword(&mut self, token: Token) -> Result<(), Error> {
        trace!("keyword: {}", token);

        const EXPECTED: [&str; 1] = ["{keyword}"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(keyword),
                ..
            } => {
                self.builder.set_keyword(keyword);
                self.state = State::End;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }
}
