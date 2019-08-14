//!
//! The boolean XOR term parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::Error;

use super::AndFactorParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    AndFactor,
    AndOperator,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::AndFactor
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
                State::AndFactor => {
                    let rpn = AndFactorParser::default().parse(stream.clone())?;
                    self.rpn.append(rpn);
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push(operator);
                    }
                    self.state = State::AndOperator;
                }
                State::AndOperator => {
                    let peek = stream.borrow_mut().peek();
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::DoubleAmpersand),
                        ..
                    })) = peek
                    {
                        let token = stream.borrow_mut().next().unwrap().unwrap();
                        log::trace!("{}", token);

                        self.operator = Some(token);
                        self.state = State::AndFactor;
                    } else {
                        self.state = State::End;
                    }
                }
                State::End => return Ok(self.rpn),
            }
        }
    }
}
