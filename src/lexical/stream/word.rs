//!
//! The word lexical analyzer.
//!

use std::convert::TryFrom;

use crate::lexical::Identifier;
use crate::lexical::IdentifierError;
use crate::lexical::Lexeme;

pub enum State {
    Start,
    Continue,
}

impl Default for State {
    fn default() -> Self {
        State::Start
    }
}

#[derive(Default)]
pub struct Analyzer {
    state: State,
}

impl Analyzer {
    pub fn analyze(mut self, bytes: &[u8]) -> (usize, Lexeme) {
        let mut size = 0;
        while let Some(byte) = bytes.get(size) {
            match self.state {
                State::Start => {
                    if !Identifier::can_start_with(*byte) {
                        break;
                    }
                    self.state = State::Continue;
                }
                State::Continue => {
                    if !Identifier::can_contain_since_index_1(*byte) {
                        break;
                    }
                }
            }

            size += 1;
        }

        let lexeme = match Identifier::try_from(&bytes[..size]) {
            Ok(identifier) => Lexeme::Identifier(identifier),
            Err(IdentifierError::IsKeyword(keyword)) => Lexeme::Keyword(keyword),
        };
        (size, lexeme)
    }
}
