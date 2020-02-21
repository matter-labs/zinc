//!
//! The lexical symbol parser.
//!

use std::str;

use crate::lexical::Symbol;

pub enum State {
    Start,
    Equals,
    Exclamation,
    Lesser,
    Greater,
    Minus,
    Dot,
    Colon,
    DoubleDot,
    Ampersand,
    VerticalBar,
    Circumflex,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ExpectedOneOf {
        expected: Vec<char>,
        found: char,
        offset: usize,
    },
    InvalidCharacter {
        found: char,
        offset: usize,
    },
    UnexpectedEnd,
}

pub fn parse(input: &str) -> Result<(usize, Symbol), Error> {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::Start => match character {
                '{' => return Ok((size + 1, Symbol::BracketCurlyLeft)),
                '}' => return Ok((size + 1, Symbol::BracketCurlyRight)),
                '[' => return Ok((size + 1, Symbol::BracketSquareLeft)),
                ']' => return Ok((size + 1, Symbol::BracketSquareRight)),
                '(' => return Ok((size + 1, Symbol::ParenthesisLeft)),
                ')' => return Ok((size + 1, Symbol::ParenthesisRight)),

                ';' => return Ok((size + 1, Symbol::Semicolon)),
                ',' => return Ok((size + 1, Symbol::Comma)),

                '+' => return Ok((size + 1, Symbol::Plus)),
                '*' => return Ok((size + 1, Symbol::Asterisk)),
                '/' => return Ok((size + 1, Symbol::Slash)),
                '%' => return Ok((size + 1, Symbol::Percent)),

                '-' => {
                    size += 1;
                    state = State::Minus;
                }
                '.' => {
                    size += 1;
                    state = State::Dot;
                }
                ':' => {
                    size += 1;
                    state = State::Colon;
                }
                '=' => {
                    size += 1;
                    state = State::Equals;
                }
                '!' => {
                    size += 1;
                    state = State::Exclamation;
                }
                '<' => {
                    size += 1;
                    state = State::Lesser;
                }
                '>' => {
                    size += 1;
                    state = State::Greater;
                }
                '&' => {
                    size += 1;
                    state = State::Ampersand;
                }
                '|' => {
                    size += 1;
                    state = State::VerticalBar;
                }
                '^' => {
                    size += 1;
                    state = State::Circumflex;
                }

                character => {
                    return Err(Error::InvalidCharacter {
                        found: character,
                        offset: size,
                    });
                }
            },
            State::Equals => {
                return match character {
                    '=' => Ok((size + 1, Symbol::DoubleEquals)),
                    '>' => Ok((size + 1, Symbol::EqualsGreater)),
                    _ => Ok((size, Symbol::Equals)),
                }
            }
            State::Exclamation => {
                return match character {
                    '=' => Ok((size + 1, Symbol::ExclamationMarkEquals)),
                    _ => Ok((size, Symbol::ExclamationMark)),
                }
            }
            State::Lesser => {
                return match character {
                    '=' => Ok((size + 1, Symbol::LesserThanEquals)),
                    _ => Ok((size, Symbol::LesserThan)),
                }
            }
            State::Greater => {
                return match character {
                    '=' => Ok((size + 1, Symbol::GreaterThanEquals)),
                    _ => Ok((size, Symbol::GreaterThan)),
                }
            }
            State::Minus => {
                return match character {
                    '>' => Ok((size + 1, Symbol::MinusGreater)),
                    _ => Ok((size, Symbol::Minus)),
                }
            }
            State::Dot => match character {
                '.' => {
                    size += 1;
                    state = State::DoubleDot;
                }
                _ => return Ok((size, Symbol::Dot)),
            },
            State::Colon => {
                return match character {
                    ':' => Ok((size + 1, Symbol::DoubleColon)),
                    _ => Ok((size, Symbol::Colon)),
                }
            }
            State::DoubleDot => {
                return match character {
                    '=' => Ok((size + 1, Symbol::DoubleDotEquals)),
                    _ => Ok((size, Symbol::DoubleDot)),
                }
            }
            State::Ampersand => {
                return match character {
                    '&' => Ok((size + 1, Symbol::DoubleAmpersand)),
                    _ => Err(Error::ExpectedOneOf {
                        expected: vec!['&'],
                        found: character,
                        offset: size,
                    }),
                }
            }
            State::VerticalBar => {
                return match character {
                    '|' => Ok((size + 1, Symbol::DoubleVerticalBar)),
                    _ => Err(Error::ExpectedOneOf {
                        expected: vec!['|'],
                        found: character,
                        offset: size,
                    }),
                }
            }
            State::Circumflex => {
                return match character {
                    '^' => Ok((size + 1, Symbol::DoubleCircumflex)),
                    _ => Err(Error::ExpectedOneOf {
                        expected: vec!['^'],
                        found: character,
                        offset: size,
                    }),
                }
            }
        }
    }

    Err(Error::UnexpectedEnd)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;
    use crate::lexical::Symbol;

    #[test]
    fn ok() {
        let input = "==";
        let expected = Ok((input.len(), Symbol::DoubleEquals));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_one_of() {
        let input = "|5";
        let expected = Err(Error::ExpectedOneOf {
            expected: vec!['|'],
            found: '5',
            offset: 1,
        });
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
        let input = "|";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(result, expected);
    }
}
