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
    rpn: LinkedList<Token>,
    operator: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        mut iterator: TokenStream,
    ) -> Result<(TokenStream, LinkedList<Token>), Error> {
        loop {
            match self.state {
                State::AndFactor => {
                    let (i, mut rpn) = AndFactorParser::default().parse(iterator)?;
                    iterator = i;
                    self.rpn.append(&mut rpn);
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push_back(operator);
                    }
                    self.state = State::AndOperator;
                }
                State::AndOperator => {
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BooleanAnd),
                        ..
                    })) = iterator.peek()
                    {
                        self.operator = Some(iterator.next().unwrap().unwrap());
                        self.state = State::AndFactor;
                    } else {
                        self.state = State::End;
                    }
                }
                State::End => return Ok((iterator, self.rpn)),
            }
        }
    }
}
