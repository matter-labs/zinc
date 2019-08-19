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
                    Some(Ok(Token {
                        lexeme: Lexeme::Keyword(Keyword::Let),
                        ..
                    })) => self.state = State::MutOrIdentifier,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["let"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::MutOrIdentifier => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Keyword(Keyword::Mut),
                        ..
                    })) => {
                        self.builder.set_mutable();
                        self.state = State::Identifier;
                    }
                    Some(Ok(Token {
                        lexeme: Lexeme::Identifier(identifier),
                        ..
                    })) => {
                        self.builder.set_identifier(identifier);
                        self.state = State::ColonOrEquals;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["mut", "{identifier}"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Identifier => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Identifier(identifier),
                        ..
                    })) => {
                        self.builder.set_identifier(identifier);
                        self.state = State::ColonOrEquals;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["{identifier}"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ColonOrEquals => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Colon),
                        ..
                    })) => self.state = State::Type,
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Equals),
                        ..
                    })) => self.state = State::Expression,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [":", "="].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Type => {
                    let r#type = TypeParser::default().parse(stream.clone())?;
                    self.builder.set_type(r#type);
                    self.state = State::Equals;
                }
                State::Equals => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Equals),
                        ..
                    })) => self.state = State::Expression,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["="].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Expression => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_expression(expression);
                    self.state = State::Semicolon;
                }
                State::Semicolon => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Semicolon),
                        ..
                    })) => self.state = State::End,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [";"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => return Ok(self.builder.finish()),
            }
        }
    }
}
