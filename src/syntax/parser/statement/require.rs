//!
//! The require statement parser.
//!

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Require;
use crate::syntax::RequireBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Keyword,
    BracketOpen,
    Expression,
    BracketClose,
    Semicolon,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Keyword
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: RequireBuilder,
}

impl Parser {
    pub fn parse(mut self, mut stream: TokenStream) -> Result<(TokenStream, Require), Error> {
        loop {
            match self.state {
                State::Keyword => match stream.next() {
                    Some(Ok(token)) => self.keyword(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::BracketOpen => match stream.next() {
                    Some(Ok(token)) => self.bracket_open(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Expression => {
                    let (s, expression) = ExpressionParser::default().parse(stream)?;
                    stream = s;
                    self.builder.set_expression(expression);
                    self.state = State::BracketClose;
                }
                State::BracketClose => match stream.next() {
                    Some(Ok(token)) => self.bracket_close(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Semicolon => match stream.next() {
                    Some(Ok(token)) => self.semicolon(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => return Ok((stream, self.builder.finish())),
            }
        }
    }

    fn keyword(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("keyword: {}", token);

        const EXPECTED: [&str; 1] = ["require"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Require),
                ..
            } => {
                self.state = State::BracketOpen;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn bracket_open(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("bracket_open: {}", token);

        const EXPECTED: [&str; 1] = ["("];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => {
                self.state = State::Expression;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn bracket_close(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("bracket_close: {}", token);

        const EXPECTED: [&str; 1] = [")"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                ..
            } => {
                self.state = State::Semicolon;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn semicolon(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("semicolon: {}", token);

        const EXPECTED: [&str; 1] = [";"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                ..
            } => {
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
