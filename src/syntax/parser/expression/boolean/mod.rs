//!
//! The boolean expression parser.
//!

mod and_factor;
mod or_term;
mod xor_term;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::syntax::ExpressionOperator;
use crate::Error;

use self::and_factor::Parser as AndFactorParser;
use self::or_term::Parser as OrTermParser;
use self::xor_term::Parser as XorTermParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    OrTerm,
    OrOperator,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::OrTerm
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
        log::trace!("expression boolean");

        loop {
            match self.state {
                State::OrTerm => {
                    let rpn = OrTermParser::default().parse(stream.clone())?;
                    self.expression.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.expression.push_operator(operator);
                    }
                    self.state = State::OrOperator;
                }
                State::OrOperator => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleVerticalBar),
                            ..
                        })) => {
                            let token = stream.borrow_mut().next().unwrap().unwrap();
                            log::trace!("{}", token);

                            self.operator = Some((ExpressionOperator::LogicalOr, token));
                            self.state = State::OrTerm;
                        }
                        _ => self.state = State::End,
                    }
                }
                State::End => return Ok(self.expression),
            }
        }
    }
}
