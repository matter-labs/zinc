//!
//! The tuple type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;
use crate::syntax::TypeParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    ParenthesisLeft,
    TypeOrParenthesisRight,
    CommaOrParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::ParenthesisLeft
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: TypeBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<Type, Error> {
        loop {
            match self.state {
                State::ParenthesisLeft => {
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::TypeOrParenthesisRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["("],
                                lexeme,
                            )))
                        }
                    }
                }
                State::TypeOrParenthesisRight => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            self.builder.set_unit_if_empty();
                            return Ok(self.builder.finish());
                        }
                        token => {
                            let array_type =
                                TypeParser::default().parse(stream.clone(), Some(token))?;
                            self.builder.push_tuple_type(array_type.variant);
                            self.state = State::CommaOrParenthesisRight;
                        }
                    }
                }
                State::CommaOrParenthesisRight => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => {
                            self.builder.set_tuple_comma();
                            self.state = State::TypeOrParenthesisRight;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ")"],
                                lexeme,
                            )))
                        }
                    }
                }
            }
        }
    }
}
