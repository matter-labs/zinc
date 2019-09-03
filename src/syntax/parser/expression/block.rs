//!
//! The block expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BlockExpression;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::syntax::StatementParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BracketOpen,
    StatementOrBracketClose,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::BracketOpen
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    block: BlockExpression,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<BlockExpression, Error> {
        loop {
            match self.state {
                State::BracketOpen => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                        location,
                    })) => {
                        self.block.location = location;
                        self.state = State::StatementOrBracketClose;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["{"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::StatementOrBracketClose => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.state = State::End;
                        }
                        Some(Ok(..)) => {
                            let (statement, is_unterminated) =
                                StatementParser::default().parse(stream.clone())?;
                            match statement {
                                Statement::Expression(expression) => {
                                    if is_unterminated {
                                        self.block.expression = Some(Box::new(expression));
                                    } else {
                                        self.block
                                            .statements
                                            .push(Statement::Expression(expression));
                                    }
                                }
                                statement => self.block.statements.push(statement),
                            }
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }
                }
                State::End => return Ok(self.block),
            }
        }
    }
}
