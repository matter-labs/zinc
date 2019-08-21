//!
//! The AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ComparisonOperandParser;
use crate::syntax::Expression;
use crate::syntax::ExpressionOperator;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    ComparisonFirstOperand,
    ComparisonOperator,
    ComparisonSecondOperand,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::ComparisonFirstOperand
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
                State::ComparisonFirstOperand => {
                    let rpn = ComparisonOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    self.state = State::ComparisonOperator;
                }
                State::ComparisonOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleEquals),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::Equal, token));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::ExclamationMarkEquals),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::NotEqual, token));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterThanEquals),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::GreaterEqual, token));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserThanEquals),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::LesserEqual, token));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::GreaterThan),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::Greater, token));
                            self.state = State::ComparisonSecondOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::LesserThan),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::Lesser, token));
                            self.state = State::ComparisonSecondOperand;
                        }
                        _ => self.state = State::End,
                    }
                }
                State::ComparisonSecondOperand => {
                    let rpn = ComparisonOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::End;
                }
                State::End => return Ok(self.expression),
            }
        }
    }
}
