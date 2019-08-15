//!
//! The arithmetic expression parser.
//!

mod factor;
mod term;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::syntax::ExpressionOperator;
use crate::Error;

use self::factor::Parser as FactorParser;
use self::term::Parser as TermParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Term,
    Operator,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Term
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
        log::trace!("expression arithmetic");

        loop {
            match self.state {
                State::Term => {
                    let rpn = TermParser::default().parse(stream.clone())?;
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
                            lexeme: Lexeme::Symbol(Symbol::Plus),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some((ExpressionOperator::Addition, token));
                            self.state = State::Term;
                        }
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::Minus),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some((ExpressionOperator::Subtraction, token));
                            self.state = State::Term;
                        }
                        _ => self.state = State::End,
                    }
                }
                State::End => return Ok(self.expression),
            }
        }
    }
}
