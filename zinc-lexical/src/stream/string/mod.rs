//!
//! The lexical string literal parser.
//!

#[cfg(test)]
mod tests;

pub mod error;
pub mod output;

use std::str;

use self::error::Error;
use self::output::Output;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    DoubleQuoteOpen,
    /// The `"` has been parsed so far.
    Character,
    /// The `\` has been found so the string will not be terminated if the next character is a `"`.
    EscapedCharacter,
}

///
/// Parses a string literal.
///
/// Example:
/// '"Zinc is the best language for ZKP"'
///
pub fn parse(input: &str) -> Result<Output, Error> {
    let mut state = State::DoubleQuoteOpen;
    let mut size = 0;
    let mut lines = 0;
    let mut column = 1;
    let mut value = String::with_capacity(64);

    loop {
        let character = input.chars().nth(size);
        match state {
            State::DoubleQuoteOpen => match character {
                Some('\"') => {
                    size += 1;
                    column += 1;
                    state = State::Character;
                }
                _ => return Err(Error::NotAString),
            },
            State::Character => match character {
                Some('\"') => {
                    size += 1;
                    return Ok(Output::new(size, value));
                }
                Some('\\') => {
                    size += 1;
                    column += 1;
                    state = State::EscapedCharacter;
                }
                Some('\n') => {
                    size += 1;
                    lines += 1;
                    column = 1;
                    state = State::EscapedCharacter;
                }
                Some(character) => {
                    value.push(character);
                    size += 1;
                    column += 1;
                }
                None => return Err(Error::UnterminatedDoubleQuote { lines, column }),
            },
            State::EscapedCharacter => match character {
                Some(character) => {
                    value.push(character);
                    size += 1;
                    column += 1;
                    state = State::Character;
                }
                None => return Err(Error::UnterminatedDoubleQuote { lines, column }),
            },
        }
    }
}
