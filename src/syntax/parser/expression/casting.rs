//!
//! The casting operand parser.
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
use crate::syntax::ExpressionParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Start,
    UnaryMulDivRemOperand,
    ParenthesisExpression,
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
        loop {
            match self.state {
                State::Start => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((ExpressionOperator::Not, token));
                            self.state = State::UnaryMulDivRemOperand;
                        }
                        Some(Ok(
                            token @ Token {
                                lexeme: Lexeme::Symbol(Symbol::Minus),
                                ..
                            },
                        )) => {
                            stream.borrow_mut().next();
                            self.operator = Some((ExpressionOperator::Negation, token));
                            self.state = State::UnaryMulDivRemOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.state = State::ParenthesisExpression;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Literal(literal),
                            ..
                        })) => {
                            let token = match stream.borrow_mut().next() {
                                Some(Ok(token)) => token,
                                Some(Err(error)) => return Err(Error::Lexical(error)),
                                None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                            };

                            self.expression
                                .push_operand((ExpressionOperand::Literal(literal), token));
                            return Ok(self.expression);
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            ..
                        })) => {
                            let token = match stream.borrow_mut().next() {
                                Some(Ok(token)) => token,
                                Some(Err(error)) => return Err(Error::Lexical(error)),
                                None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                            };

                            self.expression
                                .push_operand((ExpressionOperand::Identifier(identifier), token));
                            return Ok(self.expression);
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["!", "-", "(", "{literal}", "{identifier}"].to_vec(),
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
                State::UnaryMulDivRemOperand => {
                    let rpn = Self::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    return Ok(self.expression);
                }
                State::ParenthesisExpression => {
                    let rpn = ExpressionParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    self.state = State::ParenthesisClose;
                }
                State::ParenthesisClose => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            return Ok(self.expression);
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                [")"].to_vec(),
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
