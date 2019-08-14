//!
//! The boolean AND factor parser.
//!

use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
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
    rpn: Vec<Token>,
    operator: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, mut stream: TokenStream) -> Result<(TokenStream, Vec<Token>), Error> {
        loop {
            match self.state {
                State::Start => match stream.peek() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ExclamationMark),
                        ..
                    })) => {
                        let token = stream.next().unwrap().unwrap();
                        log::trace!("{}", token);

                        self.operator = Some(token);
                        self.state = State::UnaryExpr;
                    }
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                        ..
                    })) => {
                        let token = stream.next().unwrap().unwrap();
                        log::trace!("{}", token);

                        self.state = State::ParenthesisExpr;
                    }
                    Some(Ok(Token {
                        lexeme: Lexeme::Literal(Literal::Boolean(_)),
                        ..
                    })) => {
                        let token = stream.next().unwrap().unwrap();
                        log::trace!("{}", token);

                        self.rpn.push(token);
                        return Ok((stream, self.rpn));
                    }
                    Some(Ok(Token {
                        lexeme: Lexeme::Identifier(_),
                        ..
                    })) => {
                        let token = stream.next().unwrap().unwrap();
                        log::trace!("{}", token);

                        self.rpn.push(token);
                        return Ok((stream, self.rpn));
                    }
                    token => {
                        log::info!("{:?}", token);
                        unimplemented!();
                    }
                },
                State::UnaryExpr => {
                    let (stream, mut rpn) = Self::default().parse(stream)?;
                    self.rpn.append(&mut rpn);
                    if let Some(operator) = self.operator.take() {
                        self.rpn.push(operator);
                    }
                    return Ok((stream, self.rpn));
                }
                State::ParenthesisExpr => {
                    let (s, mut rpn) = ExpressionParser::default().parse(stream)?;
                    stream = s;
                    self.rpn.append(&mut rpn);
                    self.state = State::ParenthesisClose;
                }
                State::ParenthesisClose => {
                    if let Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                        ..
                    })) = stream.peek()
                    {
                        let token = stream.next().unwrap().unwrap();
                        log::trace!("{}", token);

                        return Ok((stream, self.rpn));
                    }
                }
            }
        }
    }
}
