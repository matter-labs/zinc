//!
//! The syntax analyzer of type.
//!

use log::*;

use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::syntax::Error as SyntaxError;
use crate::syntax::TokenIterator;
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
pub struct TypeAnalyzer {
    state: State,
    builder: TypeBuilder,
}

impl TypeAnalyzer {
    pub fn analyze(mut self, mut iterator: TokenIterator) -> Result<(TokenIterator, Type), Error> {
        loop {
            match self.state {
                State::End => return Ok((iterator, self.builder.finish())),
                _ => match iterator.next() {
                    Some(Ok(token)) => self.token(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
            }
        }
    }

    ///
    /// Routes the token to the correct handler.
    ///
    fn token(&mut self, token: Token) -> Result<(), Error> {
        match self.state {
            State::Keyword => self.keyword(token),
            _ => unreachable!(),
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
