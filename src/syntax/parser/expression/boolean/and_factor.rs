//!
//! The boolean AND factor parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::Error;

use super::Parser as ExpressionParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Start,
    UnaryExpr,
    ParenthesisExpr,
    ParenthesisClose,
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    rpn: Expression,
    operator: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::Start => {
                    const EXPECTED: [&str; 5] = ["!", "(", "true", "false", "{identifier}"];

                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some(token);
                            self.state = State::UnaryExpr;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.state = State::ParenthesisExpr;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(Literal::Boolean(_)),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.rpn.push(token);
                            return Ok(self.rpn);
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(_),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.rpn.push(token);
                            return Ok(self.rpn);
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location.to_owned(),
                                EXPECTED.to_vec(),
                                lexeme.to_owned(),
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error.to_owned())),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
                State::UnaryExpr => {
                    let rpn = Self::default().parse(stream.clone())?;
                    self.rpn.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push(operator);
                    }
                    return Ok(self.rpn);
                }
                State::ParenthesisExpr => {
                    let rpn = ExpressionParser::default().parse(stream.clone())?;
                    self.rpn.append(rpn);
                    self.state = State::ParenthesisClose;
                }
                State::ParenthesisClose => {
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                        ..
                    })) = stream.borrow_mut().peek()
                    {
                        let token = stream.borrow_mut().next().unwrap().unwrap();
                        log::trace!("{}", token);

                        return Ok(self.rpn);
                    }
                }
            }
        }
    }
}
