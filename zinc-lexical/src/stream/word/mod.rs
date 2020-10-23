//!
//! The lexical word parser.
//!

#[cfg(test)]
mod tests;

pub mod output;

use std::convert::TryFrom;
use std::str::FromStr;

use crate::token::lexeme::identifier::Error as IdentifierError;
use crate::token::lexeme::identifier::Identifier;
use crate::token::lexeme::literal::boolean::Boolean;
use crate::token::lexeme::literal::Literal;
use crate::token::lexeme::symbol::Symbol;
use crate::token::lexeme::Lexeme;

use self::output::Output;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    Start,
    /// The first character has been parsed so far.
    Continue,
}

///
/// Parses a word. The word can result into several token types:
///
/// 1. An identifier
/// 'value'
/// Any valid identifier which is not a keyword.
///
/// 2. An underscore symbol
/// '_'
/// The symbol can potentially start an identifier, but if there is no alpha symbol after the
/// underscore, it is not a valid identifier, so the underscore is treated as a symbol token.
///
/// 3. A boolean literal
/// 'true'
/// The literal is also a keyword, but is was decided to treat literals as a separate token type.
///
/// 4. A keyword
/// 'for'
/// Any keyword which is not a boolean literal.
///
pub fn parse(input: &str) -> Output {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::Start => {
                if !Identifier::can_start_with(character) {
                    break;
                }
                state = State::Continue;
            }
            State::Continue => {
                if !Identifier::can_contain_after_start(character) {
                    break;
                }
            }
        }

        size += 1;
    }

    let lexeme = match Identifier::from_str(&input[..size]) {
        Ok(identifier) => Lexeme::Identifier(identifier),
        Err(IdentifierError::IsUnderscore) => Lexeme::Symbol(Symbol::Underscore),
        Err(IdentifierError::IsKeyword(keyword)) => match Boolean::try_from(keyword) {
            Ok(boolean) => Lexeme::Literal(Literal::Boolean(boolean)),
            Err(keyword) => Lexeme::Keyword(keyword),
        },
    };
    Output::new(size, lexeme)
}
