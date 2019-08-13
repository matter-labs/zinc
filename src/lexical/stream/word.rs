//!
//! The word parser.
//!
//! The word can be a keyword, identifier, or boolean literal.
//!

use std::convert::TryFrom;

use crate::lexical::BooleanLiteral;
use crate::lexical::Identifier;
use crate::lexical::IdentifierError;
use crate::lexical::Lexeme;
use crate::lexical::Literal;

pub enum State {
    Start,
    Continue,
}

pub fn parse(bytes: &[u8]) -> (usize, Lexeme) {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => {
                if !Identifier::can_start_with(byte) {
                    break;
                }
                state = State::Continue;
            }
            State::Continue => {
                if !Identifier::can_contain_since_index_1(byte) {
                    break;
                }
            }
        }

        size += 1;
    }

    let lexeme = match Identifier::try_from(&bytes[..size]) {
        Ok(identifier) => Lexeme::Identifier(identifier),
        Err(IdentifierError::IsKeyword(keyword)) => match BooleanLiteral::try_from(keyword) {
            Ok(boolean) => Lexeme::Literal(Literal::Boolean(boolean)),
            Err(keyword) => Lexeme::Keyword(keyword),
        },
    };
    (size, lexeme)
}
