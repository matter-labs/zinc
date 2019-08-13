//!
//! The boolean expression syntax parser.
//!

mod and_factor;
mod or_term;
mod xor_term;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
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
    rpn: Vec<Token>,
    operator: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, mut iterator: TokenStream) -> Result<(TokenStream, Vec<Token>), Error> {
        loop {
            match self.state {
                State::OrTerm => {
                    let (i, mut rpn) = OrTermParser::default().parse(iterator)?;
                    iterator = i;
                    self.rpn.append(&mut rpn);
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push(operator);
                    }
                    self.state = State::OrOperator;
                }
                State::OrOperator => {
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BooleanOr),
                        ..
                    })) = iterator.peek()
                    {
                        self.operator = Some(iterator.next().unwrap().unwrap());
                        self.state = State::OrTerm;
                    } else {
                        self.state = State::End;
                    }
                }
                State::End => return Ok((iterator, self.rpn)),
            }
        }
    }
}
