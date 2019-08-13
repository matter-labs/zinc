//!
//! The boolean factor syntax parser.
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

use super::Parser as ExpressionParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Start,
    UnaryExpr,
    ParenthesisExpr,
    ParenthesisClose,
}

impl Default for State {
    fn default() -> Self {
        State::Start
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
                State::Start => match iterator.peek() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BooleanNot),
                        ..
                    })) => {
                        self.operator = Some(iterator.next().unwrap().unwrap());
                        self.state = State::UnaryExpr;
                    }
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BracketRoundOpen),
                        ..
                    })) => {
                        iterator.next();
                        self.state = State::ParenthesisExpr;
                    }
                    Some(Ok(Token {
                        lexeme: Lexeme::Literal(Literal::Boolean(_)),
                        ..
                    })) => {
                        self.rpn.push_back(iterator.next().unwrap().unwrap());
                        return Ok((iterator, self.rpn));
                    }
                    Some(Ok(Token {
                        lexeme: Lexeme::Identifier(_),
                        ..
                    })) => {
                        self.rpn.push_back(iterator.next().unwrap().unwrap());
                        return Ok((iterator, self.rpn));
                    }
                    _ => unimplemented!(),
                },
                State::UnaryExpr => {
                    let (i, mut rpn) = Self::default().parse(iterator)?;
                    iterator = i;
                    self.rpn.append(&mut rpn);
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push_back(operator);
                    }
                    return Ok((iterator, self.rpn));
                }
                State::ParenthesisExpr => {
                    let (i, mut rpn) = ExpressionParser::default().parse(iterator)?;
                    iterator = i;
                    self.rpn.append(&mut rpn);
                    self.state = State::ParenthesisClose;
                }
                State::ParenthesisClose => {
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BracketRoundClose),
                        ..
                    })) = iterator.peek()
                    {
                        iterator.next();
                        return Ok((iterator, self.rpn));
                    }
                }
            }
        }
    }
}
