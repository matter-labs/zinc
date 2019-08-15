//!
//! The arithmetic term parser.
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

use super::FactorParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Factor,
    Operator,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Factor
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
        log::trace!("expression arithmetic term");

        loop {
            match self.state {
                State::Factor => {
                    let rpn = FactorParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::Operator;
                }
                State::Operator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Asterisk),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some((ExpressionOperator::Multiplication, token));
                            self.state = State::Factor;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Slash),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some((ExpressionOperator::Division, token));
                            self.state = State::Factor;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Percent),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some((ExpressionOperator::Remainder, token));
                            self.state = State::Factor;
                        }
                        _ => self.state = State::End,
                    }
                }
                State::End => return Ok(self.expression),
            }
        }
    }
}
