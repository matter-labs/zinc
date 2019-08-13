//!
//! The statement syntax parser.
//!

use log::*;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Let,
    MutOrName,
    Name,
    ColonOrAssignment,
    Type,
    Assignment,
    Expression,
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        State::Let
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
}

impl Parser {
    pub fn parse(mut self, mut iterator: TokenStream) -> Result<(TokenStream, ()), Error> {
        loop {
            match self.state {
                State::Let => match iterator.next() {
                    Some(Ok(token)) => self.keyword(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                _ => unimplemented!(),
            }
        }
    }

    fn keyword(&mut self, token: Token) -> Result<(), Error> {
        trace!("keyword: {}", token);

        const EXPECTED: [&str; 1] = ["let"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Let),
                ..
            } => {
                self.state = State::MutOrName;
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
