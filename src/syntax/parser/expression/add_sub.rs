//!
//! The addition/subtraction operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::syntax::ExpressionOperator;
use crate::Error;

use super::MulDivRemOperandParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    MulDivRemOperand,
    MulDivRemOperator,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::MulDivRemOperand
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
                State::MulDivRemOperand => {
                    let rpn = MulDivRemOperandParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::MulDivRemOperator;
                }
                State::MulDivRemOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Asterisk),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::Multiplication, token));
                            self.state = State::MulDivRemOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Slash),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::Division, token));
                            self.state = State::MulDivRemOperand;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Percent),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            self.operator = Some((ExpressionOperator::Remainder, token));
                            self.state = State::MulDivRemOperand;
                        }
                        _ => self.state = State::End,
                    }
                }
                State::End => return Ok(self.expression),
            }
        }
    }
}
