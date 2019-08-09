//!
//! The syntax parser of witnesses.
//!

use log::*;

use crate::lexical::Delimiter;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Punctuation;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::TypeParser;
use crate::syntax::Witness;
use crate::syntax::WitnessBuilder;
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
pub struct WitnessParser {
    state: State,
    witnesses: Vec<Witness>,
    builder: WitnessBuilder,
}

impl WitnessParser {
    pub fn parse(
        mut self,
        mut iterator: TokenStream,
    ) -> Result<(TokenStream, Vec<Witness>), Error> {
        loop {
            match self.state {
                State::Keyword => match iterator.next() {
                    Some(Ok(token)) => self.keyword(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::BracketOpen => match iterator.next() {
                    Some(Ok(token)) => self.bracket_open(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementIdentifierOrBracketClose => match iterator.next() {
                    Some(Ok(token)) => self.element_identifier_or_bracket_close(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementColon => match iterator.next() {
                    Some(Ok(token)) => self.element_colon(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::ElementType => {
                    let (i, r#type) = TypeParser::default().parse(iterator)?;
                    iterator = i;
                    self.builder.set_type(r#type);
                    self.state = State::ElementSemicolon;
                }
                State::ElementSemicolon => match iterator.next() {
                    Some(Ok(token)) => self.element_semicolon(token)?,
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => {
                    return Ok((iterator, self.witnesses));
                }
            }
        }
    }

    fn keyword(&mut self, token: Token) -> Result<(), Error> {
        trace!("keyword: {}", token);

        const EXPECTED: [&str; 1] = ["witness"];

        match token {
            Token {
                lexeme: Lexeme::Keyword(Keyword::Witness),
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
                self.witnesses.push(self.builder.build());
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
