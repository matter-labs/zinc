//!
//! The arithmetic factor parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
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
    expression: Expression,
    operator: Option<(ExpressionOperator, Token)>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Expression, Error> {
        log::trace!("expression arithmetic factor");

        loop {
            match self.state {
                State::Start => {
                    const EXPECTED: [&str; 4] = ["-", "(", "{integer}", "{identifier}"];

                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Minus),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some((ExpressionOperator::Negation, token));
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
                            lexeme: Lexeme::Literal(literal),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.expression
                                .push_operand((ExpressionOperand::Literal(literal), token));
                            return Ok(self.expression);
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.expression
                                .push_operand((ExpressionOperand::Identifier(identifier), token));
                            return Ok(self.expression);
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                EXPECTED.to_vec(),
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
                State::UnaryExpr => {
                    let rpn = Self::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    return Ok(self.expression);
                }
                State::ParenthesisExpr => {
                    let rpn = ExpressionParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    self.state = State::ParenthesisClose;
                }
                State::ParenthesisClose => {
                    const EXPECTED: [&str; 1] = [")"];

                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            return Ok(self.expression);
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                EXPECTED.to_vec(),
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
            }
        }
    }
}
