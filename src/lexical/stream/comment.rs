//!
//! The comment parser.
//!

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::Comment;

pub enum State {
    Start,
    Slash,
    SingleLine,
    MultiLine,
    MultiLineStar,
}

#[derive(Debug, Fail, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
    #[fail(display = "not a comment")]
    NotAComment,
}

pub fn parse(bytes: &[u8]) -> Result<(usize, usize, usize, Comment), Error> {
    let mut state = State::Start;
    let mut size = 0;
    let mut lines = 0;
    let mut column = 1;

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => match byte {
                b'/' => {
                    size += 1;
                    column += 1;
                    state = State::Slash;
                }
                _ => return Err(Error::NotAComment),
            },
            State::Slash => match byte {
                b'/' => {
                    size += 1;
                    column += 1;
                    state = State::SingleLine;
                }
                b'*' => {
                    size += 1;
                    column += 1;
                    state = State::MultiLine;
                }
                _ => return Err(Error::NotAComment),
            },
            State::SingleLine => match byte {
                b'\n' => {
                    let comment = Comment::new(bytes[2..size].to_vec());
                    return Ok((size, lines, column, comment));
                }
                _ => {
                    size += 1;
                    column += 1;
                }
            },
            State::MultiLine => match byte {
                b'*' => {
                    size += 1;
                    column += 1;
                    state = State::MultiLineStar;
                }
                b'\n' => {
                    size += 1;
                    column = 1;
                    lines += 1;
                }
                _ => {
                    size += 1;
                    column += 1;
                }
            },
            State::MultiLineStar => match byte {
                b'/' => {
                    size += 1;
                    column += 1;
                    let comment = Comment::new(bytes[2..size - 2].to_vec());
                    return Ok((size, lines, column, comment));
                }
                _ => {
                    size += 1;
                    column += 1;
                    state = State::MultiLine;
                }
            },
        }
    }

    Err(Error::UnexpectedEnd)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;
    use crate::lexical::Comment;

    #[test]
    fn single_line_ok() {
        let input = b"//mega ultra comment text\n";
        let expected = Ok((25, 0, 26, Comment::new(b"mega ultra comment text".to_vec())));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn single_line_err_unexpected_end() {
        let input = b"//mega ultra comment text";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn multi_line_ok() {
        let input = br#"/*
    This is the mega ultra test application!
*/"#;
        let expected = Ok((
            50,
            2,
            3,
            Comment::new(b"\n    This is the mega ultra test application!\n".to_vec()),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn multi_line_err_unexpected_end() {
        let input = br#"/* This is the mega ultra test application!"#;
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_not_a_comment() {
        let input = b"not a comment text";
        let expected = Err(Error::NotAComment);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_not_a_comment_one_slash() {
        let input = b"/almost a comment text";
        let expected = Err(Error::NotAComment);
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
