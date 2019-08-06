//!
//! The lexical analyzer.
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
    pub fn analyze(mut self, bytes: &[u8], start: usize) -> (usize, Lexeme) {
        let mut end = start;
        while let Some(byte) = bytes.get(end) {
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
            end += 1;
        }

        let lexeme = match Identifier::try_from(&bytes[start..end]) {
            Ok(identifier) => Lexeme::Identifier(identifier),
            Err(IdentifierError::IsKeyword(keyword)) => Lexeme::Keyword(keyword),
        };
        (end, lexeme)
    }
}
