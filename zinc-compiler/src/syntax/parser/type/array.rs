//!
//! The array type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;
use crate::syntax::TypeParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BracketSquareLeft,
    Type,
    Semicolon,
    SizeExpression,
    BracketSquareRight,
}

impl Default for State {
    fn default() -> Self {
        State::BracketSquareLeft
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
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
                State::BracketSquareLeft => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Type;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["["],
                                lexeme,
                            )))
                        }
                    }
                }
                State::Type => {
                    let (array_type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_array_type_variant(array_type.variant);
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => {
                            self.state = State::SizeExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![";"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::SizeExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_array_size_expression(expression);
                    self.state = State::BracketSquareRight;
                }
                State::BracketSquareRight => {
                    return match crate::syntax::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => Ok(self.builder.finish()),
                        Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            vec!["]"],
                            lexeme,
                        ))),
                    }
                }
            }
        }
    }
}
