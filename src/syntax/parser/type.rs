//!
//! The type syntax parser.
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
    Name,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Name
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: TypeBuilder,
}

impl Parser {
    pub fn parse(mut self, mut iterator: TokenStream) -> Result<(TokenStream, Type), Error> {
        loop {
            match self.state {
                State::Name => match iterator.next() {
                    Some(Ok(token)) => self.name(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => return Ok((iterator, self.builder.finish())),
            }
        }
    }

    fn name(&mut self, token: Token) -> Result<(), Error> {
        trace!("name: {}", token);

        const EXPECTED: [&str; 1] = ["{type}"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(keyword),
                ..
            } => {
                self.builder.set_name(keyword);
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
