//!
//! The let statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Let;
use crate::syntax::LetBuilder;
use crate::syntax::TypeParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Keyword,
    MutOrIdentifier,
    Identifier,
    ColonOrEquals,
    Type,
    Equals,
    Expression,
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
    builder: LetBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Let, Error> {
        loop {
            match self.state {
                State::Keyword => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.keyword(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::MutOrIdentifier => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.mut_or_identifier(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Identifier => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.identifier(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ColonOrEquals => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.colon_or_equals(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Type => {
                    let r#type = TypeParser::default().parse(stream.clone())?;
                    self.builder.set_type(r#type);
                    self.state = State::Equals;
                }
                State::Equals => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.equals(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Expression => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_expression(expression);
                    self.state = State::Semicolon;
                }
                State::Semicolon => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.semicolon(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => return Ok(self.builder.finish()),
            }
        }
    }

    fn keyword(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("keyword: {}", token);

        const EXPECTED: [&str; 1] = ["let"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Let),
                ..
            } => {
                self.state = State::MutOrIdentifier;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn mut_or_identifier(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("mut_or_identifier: {}", token);

        const EXPECTED: [&str; 2] = ["mut", "{identifier}"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Mut),
                ..
            } => {
                self.builder.set_mutable();
                self.state = State::Identifier;
                Ok(())
            }
            Token {
                lexeme: Lexeme::Identifier(identifier),
                ..
            } => {
                self.builder.set_identifier(identifier);
                self.state = State::ColonOrEquals;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn identifier(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("identifier: {}", token);

        const EXPECTED: [&str; 1] = ["{identifier}"];

        match token {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                ..
            } => {
                self.builder.set_identifier(identifier);
                self.state = State::ColonOrEquals;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn colon_or_equals(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("colon_or_equals: {}", token);

        const EXPECTED: [&str; 2] = [":", "="];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Colon),
                ..
            } => {
                self.state = State::Type;
                Ok(())
            }
            Token {
                lexeme: Lexeme::Symbol(Symbol::Equals),
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

    fn equals(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("equals: {}", token);

        const EXPECTED: [&str; 1] = ["="];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Equals),
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
