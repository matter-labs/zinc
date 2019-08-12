//!
//! The inputs syntax parser.
//!

use log::*;

use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::lexical::{Keyword, Symbol};
use crate::syntax::Error as SyntaxError;
use crate::syntax::Input;
use crate::syntax::InputBuilder;
use crate::syntax::TypeParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Keyword,
    BracketOpen,
    ElementIdentifierOrBracketClose,
    ElementColon,
    ElementType,
    ElementSemicolon,
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
    inputs: Vec<Input>,
    builder: InputBuilder,
}

impl Parser {
    pub fn parse(mut self, mut iterator: TokenStream) -> Result<(TokenStream, Vec<Input>), Error> {
        loop {
            match self.state {
                State::Keyword => match iterator.next() {
                    Some(Ok(token)) => self.keyword(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::BracketOpen => match iterator.next() {
                    Some(Ok(token)) => self.bracket_open(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementIdentifierOrBracketClose => match iterator.next() {
                    Some(Ok(token)) => self.element_identifier_or_bracket_close(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementColon => match iterator.next() {
                    Some(Ok(token)) => self.element_colon(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementType => {
                    let (i, r#type) = TypeParser::default().parse(iterator)?;
                    iterator = i;
                    self.builder.set_type(r#type);
                    self.state = State::ElementSemicolon;
                }
                State::ElementSemicolon => match iterator.next() {
                    Some(Ok(token)) => self.element_semicolon(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => {
                    return Ok((iterator, self.inputs));
                }
            }
        }
    }

    fn keyword(&mut self, token: Token) -> Result<(), Error> {
        trace!("keyword: {}", token);

        const EXPECTED: [&str; 1] = ["inputs"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Inputs),
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
        trace!("bracket_open: {}", token);

        const EXPECTED: [&str; 1] = ["{"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyOpen),
                ..
            } => {
                self.state = State::ElementIdentifierOrBracketClose;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn element_identifier_or_bracket_close(&mut self, token: Token) -> Result<(), Error> {
        trace!("element_identifier_or_bracket_close: {}", token);

        const EXPECTED: [&str; 2] = ["{identifier}", "}"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyClose),
                ..
            } => {
                self.state = State::End;
                Ok(())
            }
            Token {
                lexeme: Lexeme::Identifier(identifier),
                ..
            } => {
                self.builder.set_identifier(identifier);
                self.state = State::ElementColon;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn element_colon(&mut self, token: Token) -> Result<(), Error> {
        trace!("element_colon: {}", token);

        const EXPECTED: [&str; 1] = [":"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Colon),
                ..
            } => {
                self.state = State::ElementType;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn element_semicolon(&mut self, token: Token) -> Result<(), Error> {
        trace!("element_semicolon: {}", token);

        const EXPECTED: [&str; 1] = [";"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                ..
            } => {
                self.inputs.push(self.builder.build());
                self.state = State::ElementIdentifierOrBracketClose;
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
