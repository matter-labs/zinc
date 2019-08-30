//!
//! The statement parser.
//!

mod debug;
mod r#let;
mod require;

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Statement;
use crate::Error;

use self::debug::Parser as DebugParser;
use self::r#let::Parser as LetParser;
use self::require::Parser as RequireParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Statement,
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        State::Statement
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    statement: Option<Statement>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Statement, Error> {
        loop {
            match self.state {
                State::Statement => {
                    let peek = stream.borrow_mut().peek();
                    self.statement = Some(match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Let),
                            ..
                        })) => LetParser::default()
                            .parse(stream.clone())
                            .map(Statement::Let),
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Require),
                            ..
                        })) => RequireParser::default()
                            .parse(stream.clone())
                            .map(Statement::Require),
                        Some(Ok(Token {
                            lexeme: Lexeme::Keyword(Keyword::Debug),
                            ..
                        })) => DebugParser::default()
                            .parse(stream.clone())
                            .map(Statement::Debug),
                        Some(Ok(Token { lexeme, location })) => {
                            Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                ["let", "require", "debug"].to_vec(),
                                lexeme,
                            )))
                        }
                        Some(Err(error)) => Err(Error::Lexical(error)),
                        None => Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                    }?);
                    self.state = State::Semicolon;
                }
                State::Semicolon => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Symbol(Symbol::Semicolon),
                        ..
                    })) => return Ok(self.statement.take().expect("Option state bug")),
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            [";"].to_vec(),
                            lexeme,
                        )));
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
            }
        }
    }
}
