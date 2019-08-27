//!
//! The require statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
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
    CommaOrBracketClose,
    Tag,
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
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Require, Error> {
        loop {
            match self.state {
                State::Keyword => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Keyword(Keyword::Require),
                        location,
                    })) => {
                        self.builder.set_location(location);
                        self.state = State::BracketOpen;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["require"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::BracketOpen => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                        ..
                    })) => self.state = State::Expression,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["("].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Expression => {
                    let expression = ExpressionParser::default().parse(stream.clone())?;
                    self.builder.set_expression(expression);
                    self.state = State::CommaOrBracketClose;
                }
                State::CommaOrBracketClose => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Comma),
                        ..
                    })) => self.state = State::Tag,
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                        ..
                    })) => self.state = State::Semicolon,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [",", ")"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::Tag => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Literal(Literal::String(tag)),
                        ..
                    })) => {
                        self.builder.set_tag(tag);
                        self.state = State::BracketClose;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["{string}"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::BracketClose => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                        ..
                    })) => self.state = State::Semicolon,
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [")"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
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
