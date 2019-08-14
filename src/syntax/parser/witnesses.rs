//!
//! The witnesses parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::TypeParser;
use crate::syntax::Witness;
use crate::syntax::WitnessBuilder;
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
    witnesses: Vec<Witness>,
    builder: WitnessBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Vec<Witness>, Error> {
        match stream.borrow_mut().peek() {
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(Keyword::Witness),
                ..
            })) => {}
            _ => return Ok(self.witnesses),
        }

        loop {
            match self.state {
                State::Keyword => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.keyword(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::BracketOpen => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.bracket_open(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementIdentifierOrBracketClose => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.element_identifier_or_bracket_close(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementColon => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.element_colon(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementType => {
                    let r#type = TypeParser::default().parse(stream.clone())?;
                    self.builder.set_type(r#type);
                    self.state = State::ElementSemicolon;
                }
                State::ElementSemicolon => match stream.borrow_mut().next() {
                    Some(Ok(token)) => self.element_semicolon(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => return Ok(self.witnesses),
            }
        }
    }

    fn keyword(&mut self, token: Token) -> Result<(), Error> {
        log::trace!("keyword: {}", token);

        const EXPECTED: [&str; 1] = ["witness"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Witness),
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

        const EXPECTED: [&str; 1] = ["{"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
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
        log::trace!("element_identifier_or_bracket_close: {}", token);

        const EXPECTED: [&str; 2] = ["{identifier}", "}"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
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
        log::trace!("element_colon: {}", token);

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
        log::trace!("element_semicolon: {}", token);

        const EXPECTED: [&str; 1] = [";"];

        match token {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Semicolon),
                ..
            } => {
                self.witnesses.push(self.builder.build());
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
