//!
//! The lexical comment parser.
//!

#[cfg(test)]
mod tests;

pub mod error;
pub mod output;

use crate::token::lexeme::comment::Comment;

use self::error::Error;
use self::output::Output;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    Start,
    /// The `/` has been parsed so far.
    Slash,
    /// The `//` has been parsed so far.
    SingleLine,
    /// The `/*` has been parsed so far.
    MultiLine,
    /// The `/* ... *` has been parsed so far.
    MultiLineStar,
}

///
/// Parses a comment.
///
/// Comments can be of two types:
///
/// 1. Single-line
/// '// comment'
///
/// 2. Multi-line
/// /*
///     comment
/// */
///
pub fn parse(input: &str) -> Result<Output, Error> {
    let mut state = State::Start;
    let mut size = 0;
    let mut lines = 0;
    let mut column = 1;

    loop {
        let character = input.chars().nth(size);
        match state {
            State::Start => match character {
                Some('/') => {
                    size += 1;
                    column += 1;
                    state = State::Slash;
                }
                Some(_) => return Err(Error::NotAComment),
                None => return Err(Error::UnterminatedBlock { lines, column }),
            },
            State::Slash => match character {
                Some('/') => {
                    size += 1;
                    column += 1;
                    state = State::SingleLine;
                }
                Some('*') => {
                    size += 1;
                    column += 1;
                    state = State::MultiLine;
                }
                Some(_) => return Err(Error::NotAComment),
                None => return Err(Error::UnterminatedBlock { lines, column }),
            },
            State::SingleLine => match character {
                Some('\n') => {
                    size += 1;
                    column += 1;
                    lines += 1;
                    let comment = Comment::new_line(input[2..size - 1].to_owned());
                    return Ok(Output::new(size, lines, column, comment));
                }
                Some(_) => {
                    size += 1;
                    column += 1;
                }
                None => {
                    let comment = Comment::new_line(input[2..size].to_owned());
                    return Ok(Output::new(size, lines, column, comment));
                }
            },
            State::MultiLine => match character {
                Some('*') => {
                    size += 1;
                    column += 1;
                    state = State::MultiLineStar;
                }
                Some('\n') => {
                    size += 1;
                    column = 1;
                    lines += 1;
                }
                Some(_) => {
                    size += 1;
                    column += 1;
                }
                None => return Err(Error::UnterminatedBlock { lines, column }),
            },
            State::MultiLineStar => match character {
                Some('/') => {
                    size += 1;
                    column += 1;
                    let comment = Comment::new_block(input[2..size - 2].to_owned());
                    return Ok(Output::new(size, lines, column, comment));
                }
                Some(_) => {
                    size += 1;
                    column += 1;
                    state = State::MultiLine;
                }
                None => return Err(Error::UnterminatedBlock { lines, column }),
            },
        }
    }
}
