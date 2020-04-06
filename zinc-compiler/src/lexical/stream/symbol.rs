//!
//! The lexical symbol parser.
//!

use std::str;

use crate::lexical::token::lexeme::symbol::Symbol;

pub enum State {
    Start,
    Equals,
    Exclamation,
    Lesser,
    Greater,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Dot,
    Colon,
    Ampersand,
    VerticalBar,
    Circumflex,
    DoubleDot,
    DoubleLesser,
    DoubleGreater,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidCharacter { found: char, offset: usize },
    UnexpectedEnd,
}

pub fn parse(input: &str) -> Result<(usize, Symbol), Error> {
    let mut state = State::Start;
    let mut size = 0;

    loop {
        let character = input.chars().nth(size);
        match state {
            State::Start => match character {
                Some('{') => return Ok((size + 1, Symbol::BracketCurlyLeft)),
                Some('}') => return Ok((size + 1, Symbol::BracketCurlyRight)),
                Some('[') => return Ok((size + 1, Symbol::BracketSquareLeft)),
                Some(']') => return Ok((size + 1, Symbol::BracketSquareRight)),
                Some('(') => return Ok((size + 1, Symbol::ParenthesisLeft)),
                Some(')') => return Ok((size + 1, Symbol::ParenthesisRight)),

                Some(';') => return Ok((size + 1, Symbol::Semicolon)),
                Some(',') => return Ok((size + 1, Symbol::Comma)),

                Some('~') => return Ok((size + 1, Symbol::Tilde)),

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
                    Some('=') => Ok((size + 1, Symbol::DoubleEquals)),
                    Some('>') => Ok((size + 1, Symbol::EqualsGreater)),
                    _ => Ok((size, Symbol::Equals)),
                }
            }
            State::Exclamation => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::ExclamationMarkEquals)),
                    _ => Ok((size, Symbol::ExclamationMark)),
                }
            }
            State::Lesser => match character {
                Some('=') => return Ok((size + 1, Symbol::LesserEquals)),
                Some('<') => {
                    size += 1;
                    state = State::DoubleLesser;
                }
                _ => return Ok((size, Symbol::Lesser)),
            },
            State::Greater => match character {
                Some('=') => return Ok((size + 1, Symbol::GreaterEquals)),
                Some('>') => {
                    size += 1;
                    state = State::DoubleGreater;
                }
                _ => return Ok((size, Symbol::Greater)),
            },
            State::Plus => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::PlusEquals)),
                    _ => Ok((size, Symbol::Plus)),
                }
            }
            State::Minus => {
                return match character {
                    Some('>') => Ok((size + 1, Symbol::MinusGreater)),
                    Some('=') => Ok((size + 1, Symbol::MinusEquals)),
                    _ => Ok((size, Symbol::Minus)),
                }
            }
            State::Asterisk => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::AsteriskEquals)),
                    _ => Ok((size, Symbol::Asterisk)),
                }
            }
            State::Slash => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::SlashEquals)),
                    _ => Ok((size, Symbol::Slash)),
                }
            }
            State::Percent => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::PercentEquals)),
                    _ => Ok((size, Symbol::Percent)),
                }
            }
            State::Ampersand => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::AmpersandEquals)),
                    Some('&') => Ok((size + 1, Symbol::DoubleAmpersand)),
                    _ => Ok((size, Symbol::Ampersand)),
                }
            }
            State::VerticalBar => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::VerticalBarEquals)),
                    Some('|') => Ok((size + 1, Symbol::DoubleVerticalBar)),
                    _ => Ok((size, Symbol::VerticalBar)),
                }
            }
            State::Circumflex => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::CircumflexEquals)),
                    Some('^') => Ok((size + 1, Symbol::DoubleCircumflex)),
                    _ => Ok((size, Symbol::Circumflex)),
                }
            }
            State::Dot => match character {
                Some('.') => {
                    size += 1;
                    state = State::DoubleDot;
                }
                _ => return Ok((size, Symbol::Dot)),
            },
            State::Colon => {
                return match character {
                    Some(':') => Ok((size + 1, Symbol::DoubleColon)),
                    _ => Ok((size, Symbol::Colon)),
                }
            }
            State::DoubleDot => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::DoubleDotEquals)),
                    _ => Ok((size, Symbol::DoubleDot)),
                }
            }
            State::DoubleLesser => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::DoubleLesserEquals)),
                    _ => Ok((size, Symbol::DoubleLesser)),
                }
            }
            State::DoubleGreater => {
                return match character {
                    Some('=') => Ok((size + 1, Symbol::DoubleGreaterEquals)),
                    _ => Ok((size, Symbol::DoubleGreater)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;
    use crate::lexical::token::lexeme::symbol::Symbol;

    #[test]
    fn ok() {
        let input = "==";
        let expected = Ok((input.len(), Symbol::DoubleEquals));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_invalid_character() {
        let input = "#";
        let expected = Err(Error::InvalidCharacter {
            found: '#',
            offset: 0,
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_unexpected_end() {
        let input = "";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(result, expected);
    }
}
