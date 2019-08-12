//!
//! The boolean term syntax parser.
//!

use std::collections::LinkedList;

use log::*;

use crate::lexical::BooleanLiteral;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
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
    rpn: LinkedList<Lexeme>,
    operator: Option<Symbol>,
}

impl Parser {
    pub fn parse(
        mut self,
        mut iterator: TokenStream,
    ) -> Result<(TokenStream, LinkedList<Lexeme>), Error> {
        loop {
            match self.state {
                State::Factor => {
                    let (i, boolean) = FactorParser::default().parse(iterator)?;
                    iterator = i;
                    self.rpn
                        .push_back(Lexeme::Literal(Literal::Boolean(boolean)));
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push_back(Lexeme::Symbol(operator));
                    }
                    self.state = State::Operator;
                }
                State::Operator => {
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BooleanAnd),
                        ..
                    })) = iterator.peek()
                    {
                        iterator.next();
                        self.operator = Some(Symbol::BooleanAnd);
                        self.state = State::Factor;
                    } else {
                        self.state = State::End;
                    }
                }
                State::End => return Ok((iterator, self.rpn)),
            }
        }
    }
}
