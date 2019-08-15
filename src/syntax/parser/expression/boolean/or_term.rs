//!
//! The boolean OR term parser.
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

use super::XorTermParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    XorTerm,
    XorOperator,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::XorTerm
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
        log::trace!("expression boolean OR term");

        loop {
            match self.state {
                State::XorTerm => {
                    let rpn = XorTermParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::XorOperator;
                }
                State::XorOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleCircumflex),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some((ExpressionOperator::LogicalXor, token));
                            self.state = State::XorTerm;
                        }
                        _ => self.state = State::End,
                    }
                }
                State::End => return Ok(self.expression),
            }
        }
    }
}
