//!
//! The lexical symbol parser.
//!

#[cfg(test)]
mod tests;

pub mod error;
pub mod output;

use std::str;

use crate::token::lexeme::symbol::Symbol;

use self::error::Error;
use self::output::Output;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    Start,
    /// The `=` has been parsed so far.
    Equals,
    /// The `~` has been parsed so far.
    Exclamation,
    /// The `<` has been parsed so far.
    Lesser,
    /// The `>` has been parsed so far.
    Greater,
    /// The `+` has been parsed so far.
    Plus,
    /// The `-` has been parsed so far.
    Minus,
    /// The `*` has been parsed so far.
    Asterisk,
    /// The `/` has been parsed so far.
    Slash,
    /// The `%` has been parsed so far.
    Percent,
    /// The `.` has been parsed so far.
    Dot,
    /// The `:` has been parsed so far.
    Colon,
    /// The `&` has been parsed so far.
    Ampersand,
    /// The `|` has been parsed so far.
    VerticalBar,
    /// The `^` has been parsed so far.
    Circumflex,
    /// The `..` has been parsed so far.
    DoubleDot,
    /// The `<<` has been parsed so far.
    DoubleLesser,
    /// The `>>` has been parsed so far.
    DoubleGreater,
}

///
/// The parser error.
///

///
/// Parses a symbol.
///
/// Returns the symbol and its size.
///
pub fn parse(input: &str) -> Result<Output, Error> {
    let mut state = State::Start;
    let mut size = 0;

    loop {
        let character = input.chars().nth(size);
        match state {
            State::Start => match character {
                Some('{') => return Ok(Output::new(size + 1, Symbol::BracketCurlyLeft)),
                Some('}') => return Ok(Output::new(size + 1, Symbol::BracketCurlyRight)),
                Some('[') => return Ok(Output::new(size + 1, Symbol::BracketSquareLeft)),
                Some(']') => return Ok(Output::new(size + 1, Symbol::BracketSquareRight)),
                Some('(') => return Ok(Output::new(size + 1, Symbol::ParenthesisLeft)),
                Some(')') => return Ok(Output::new(size + 1, Symbol::ParenthesisRight)),

                Some(';') => return Ok(Output::new(size + 1, Symbol::Semicolon)),
                Some(',') => return Ok(Output::new(size + 1, Symbol::Comma)),

                Some('~') => return Ok(Output::new(size + 1, Symbol::Tilde)),

                Some('#') => return Ok(Output::new(size + 1, Symbol::Number)),

                Some('+') => {
                    size += 1;
                    state = State::Plus;
                }
                Some('-') => {
                    size += 1;
                    state = State::Minus;
                }
                Some('*') => {
                    size += 1;
                    state = State::Asterisk;
                }
                Some('/') => {
                    size += 1;
                    state = State::Slash;
                }
                Some('%') => {
                    size += 1;
                    state = State::Percent;
                }
                Some('.') => {
                    size += 1;
                    state = State::Dot;
                }
                Some(':') => {
                    size += 1;
                    state = State::Colon;
                }
                Some('=') => {
                    size += 1;
                    state = State::Equals;
                }
                Some('!') => {
                    size += 1;
                    state = State::Exclamation;
                }
                Some('<') => {
                    size += 1;
                    state = State::Lesser;
                }
                Some('>') => {
                    size += 1;
                    state = State::Greater;
                }
                Some('&') => {
                    size += 1;
                    state = State::Ampersand;
                }
                Some('|') => {
                    size += 1;
                    state = State::VerticalBar;
                }
                Some('^') => {
                    size += 1;
                    state = State::Circumflex;
                }
                Some(character) => {
                    return Err(Error::InvalidCharacter {
                        found: character,
                        offset: size,
                    });
                }
                None => return Err(Error::UnexpectedEnd),
            },
            State::Equals => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::DoubleEquals)),
                    Some('>') => Ok(Output::new(size + 1, Symbol::EqualsGreater)),
                    _ => Ok(Output::new(size, Symbol::Equals)),
                }
            }
            State::Exclamation => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::ExclamationMarkEquals)),
                    _ => Ok(Output::new(size, Symbol::ExclamationMark)),
                }
            }
            State::Lesser => match character {
                Some('=') => return Ok(Output::new(size + 1, Symbol::LesserEquals)),
                Some('<') => {
                    size += 1;
                    state = State::DoubleLesser;
                }
                _ => return Ok(Output::new(size, Symbol::Lesser)),
            },
            State::Greater => match character {
                Some('=') => return Ok(Output::new(size + 1, Symbol::GreaterEquals)),
                Some('>') => {
                    size += 1;
                    state = State::DoubleGreater;
                }
                _ => return Ok(Output::new(size, Symbol::Greater)),
            },
            State::Plus => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::PlusEquals)),
                    _ => Ok(Output::new(size, Symbol::Plus)),
                }
            }
            State::Minus => {
                return match character {
                    Some('>') => Ok(Output::new(size + 1, Symbol::MinusGreater)),
                    Some('=') => Ok(Output::new(size + 1, Symbol::MinusEquals)),
                    _ => Ok(Output::new(size, Symbol::Minus)),
                }
            }
            State::Asterisk => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::AsteriskEquals)),
                    _ => Ok(Output::new(size, Symbol::Asterisk)),
                }
            }
            State::Slash => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::SlashEquals)),
                    _ => Ok(Output::new(size, Symbol::Slash)),
                }
            }
            State::Percent => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::PercentEquals)),
                    _ => Ok(Output::new(size, Symbol::Percent)),
                }
            }
            State::Ampersand => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::AmpersandEquals)),
                    Some('&') => Ok(Output::new(size + 1, Symbol::DoubleAmpersand)),
                    _ => Ok(Output::new(size, Symbol::Ampersand)),
                }
            }
            State::VerticalBar => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::VerticalBarEquals)),
                    Some('|') => Ok(Output::new(size + 1, Symbol::DoubleVerticalBar)),
                    _ => Ok(Output::new(size, Symbol::VerticalBar)),
                }
            }
            State::Circumflex => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::CircumflexEquals)),
                    Some('^') => Ok(Output::new(size + 1, Symbol::DoubleCircumflex)),
                    _ => Ok(Output::new(size, Symbol::Circumflex)),
                }
            }
            State::Dot => match character {
                Some('.') => {
                    size += 1;
                    state = State::DoubleDot;
                }
                _ => return Ok(Output::new(size, Symbol::Dot)),
            },
            State::Colon => {
                return match character {
                    Some(':') => Ok(Output::new(size + 1, Symbol::DoubleColon)),
                    _ => Ok(Output::new(size, Symbol::Colon)),
                }
            }
            State::DoubleDot => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::DoubleDotEquals)),
                    _ => Ok(Output::new(size, Symbol::DoubleDot)),
                }
            }
            State::DoubleLesser => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::DoubleLesserEquals)),
                    _ => Ok(Output::new(size, Symbol::DoubleLesser)),
                }
            }
            State::DoubleGreater => {
                return match character {
                    Some('=') => Ok(Output::new(size + 1, Symbol::DoubleGreaterEquals)),
                    _ => Ok(Output::new(size, Symbol::DoubleGreater)),
                }
            }
        }
    }
}
