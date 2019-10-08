//!
//! The tuple type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;
use crate::syntax::TypeParser;
use crate::Error;

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
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Type, Error> {
        loop {
            match self.state {
                State::ParenthesisLeft => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        })) => {
                            self.builder.set_location(location);
                            self.state = State::TypeOrParenthesisRight;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["("],
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::TypeOrParenthesisRight => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.builder.set_unit_if_empty();
                            return Ok(self.builder.finish());
                        }
                        Some(Ok(..)) => {
                            let array_type = TypeParser::default().parse(stream.clone())?;
                            self.builder.push_tuple_type(array_type.variant);
                            self.state = State::CommaOrParenthesisRight;
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::CommaOrParenthesisRight => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        })) => {
                            self.builder.set_tuple_comma();
                            self.state = State::TypeOrParenthesisRight;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => return Ok(self.builder.finish()),
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ")"],
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
            }
        }
    }
}
