//!
//! The boolean OR term parser.
//!

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
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
    rpn: Vec<Token>,
    operator: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, mut stream: TokenStream) -> Result<(TokenStream, Vec<Token>), Error> {
        loop {
            match self.state {
                State::XorTerm => {
                    let (s, mut rpn) = XorTermParser::default().parse(stream)?;
                    stream = s;
                    self.rpn.append(&mut rpn);
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push(operator);
                    }
                    self.state = State::XorOperator;
                }
                State::XorOperator => {
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::DoubleCircumflex),
                        ..
                    })) = stream.peek()
                    {
                        let token = stream.next().unwrap().unwrap();
                        log::trace!("{}", token);

                        self.operator = Some(token);
                        self.state = State::XorTerm;
                    } else {
                        self.state = State::End;
                    }
                }
                State::End => return Ok((stream, self.rpn)),
            }
        }
    }
}
