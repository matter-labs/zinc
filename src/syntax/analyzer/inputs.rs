//!
//! The syntax analyzer of inputs.
//!

use log::*;

use crate::lexical::Delimiter;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Punctuation;
use crate::lexical::Token;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Input;
use crate::syntax::InputBuilder;
use crate::syntax::TokenIterator;
use crate::syntax::TypeAnalyzer;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Keyword,
    BracketOpen,
    ElementIdentifierOrBracketClose,
    ElementColon,
    ElementType,
    ElementSemicolon,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Keyword
    }
}

#[derive(Default)]
pub struct InputsAnalyzer {
    state: State,
    inputs: Vec<Input>,
    builder: InputBuilder,
}

impl InputsAnalyzer {
    pub fn analyze(
        mut self,
        mut iterator: TokenIterator,
    ) -> Result<(TokenIterator, Vec<Input>), Error> {
        loop {
            match self.state {
                State::ElementType => {
                    let (i, r#type) = TypeAnalyzer::default().analyze(iterator)?;
                    iterator = i;
                    self.builder.set_type(r#type);
                    self.state = State::ElementSemicolon;
                }
                State::End => {
                    return Ok((iterator, self.inputs));
                }
                _ => match iterator.next() {
                    Some(Ok(token)) => self.token(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
            }
        }
    }

    ///
    /// Routes the token to the correct handler.
    ///
    fn token(&mut self, token: Token) -> Result<(), Error> {
        match self.state {
            State::Keyword => self.keyword(token),
            State::BracketOpen => self.bracket_open(token),
            State::ElementIdentifierOrBracketClose => {
                self.element_identifier_or_bracket_close(token)
            }
            State::ElementColon => self.element_colon(token),
            State::ElementSemicolon => self.element_semicolon(token),
            _ => unreachable!(),
        }
    }

    fn keyword(&mut self, token: Token) -> Result<(), Error> {
        trace!("keyword: {}", token);

        const EXPECTED: [&str; 1] = ["inputs"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Inputs),
                ..
            } => {
                self.state = State::BracketOpen;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn bracket_open(&mut self, token: Token) -> Result<(), Error> {
        trace!("bracket_open: {}", token);

        const EXPECTED: [&str; 1] = ["{"];

        match token {
            Token {
                lexeme: Lexeme::Delimiter(Delimiter::BracketCurlyOpen),
                ..
            } => {
                self.state = State::ElementIdentifierOrBracketClose;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn element_identifier_or_bracket_close(&mut self, token: Token) -> Result<(), Error> {
        trace!("element_identifier_or_bracket_close: {}", token);

        const EXPECTED: [&str; 2] = ["{identifier}", "}"];

        match token {
            Token {
                lexeme: Lexeme::Delimiter(Delimiter::BracketCurlyClose),
                ..
            } => {
                self.state = State::End;
                Ok(())
            }
            Token {
                lexeme: Lexeme::Identifier(identifier),
                ..
            } => {
                self.builder.set_identifier(identifier);
                self.state = State::ElementColon;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn element_colon(&mut self, token: Token) -> Result<(), Error> {
        trace!("element_colon: {}", token);

        const EXPECTED: [&str; 1] = [":"];

        match token {
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Colon),
                ..
            } => {
                self.state = State::ElementType;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }

    fn element_semicolon(&mut self, token: Token) -> Result<(), Error> {
        trace!("element_semicolon: {}", token);

        const EXPECTED: [&str; 1] = [";"];

        match token {
            Token {
                lexeme: Lexeme::Punctuation(Punctuation::Semicolon),
                ..
            } => {
                self.inputs.push(self.builder.build());
                self.state = State::ElementIdentifierOrBracketClose;
                Ok(())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                EXPECTED.to_vec(),
                lexeme,
            ))),
        }
    }
}
