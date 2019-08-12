//!
//! The boolean expression syntax parser.
//!

mod factor;
mod term;

use std::collections::LinkedList;

use log::*;

use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
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
                State::Term => {
                    let (i, mut rpn) = TermParser::default().parse(iterator)?;
                    iterator = i;
                    self.rpn.append(&mut rpn);
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push_back(Lexeme::Symbol(operator));
                    }
                    self.state = State::Operator;
                }
                State::Operator => {
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BooleanOr),
                        ..
                    })) = iterator.peek()
                    {
                        iterator.next();
                        self.operator = Some(Symbol::BooleanOr);
                        self.state = State::Term;
                    } else {
                        self.state = State::End;
                    }
                }
                State::End => return Ok((iterator, self.rpn)),
            }
        }
    }
}
