//!
//! The lexical comment parser.
//!

use failure::Fail;

use crate::lexical::token::lexeme::comment::Comment;

pub enum State {
    Start,
    Slash,
    SingleLine,
    MultiLine,
    MultiLineStar,
}

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "unexpected end")]
    UnexpectedEnd,
    #[fail(display = "not a comment")]
    NotAComment,
}

pub fn parse(input: &str) -> Result<(usize, usize, usize, Comment), Error> {
    let mut state = State::Start;
    let mut size = 0;
    let mut lines = 0;
    let mut column = 1;

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::Start => match character {
                '/' => {
                    size += 1;
                    column += 1;
                    state = State::Slash;
                }
                _ => return Err(Error::NotAComment),
            },
            State::Slash => match character {
                '/' => {
                    size += 1;
                    column += 1;
                    state = State::SingleLine;
                }
                '*' => {
                    size += 1;
                    column += 1;
                    state = State::MultiLine;
                }
                _ => return Err(Error::NotAComment),
            },
            State::SingleLine => match character {
                '\n' => {
                    let comment = Comment::new(input[2..size].to_owned());
                    return Ok((size, lines, column, comment));
                }
                _ => {
                    size += 1;
                    column += 1;
                }
            },
            State::MultiLine => match character {
                '*' => {
                    size += 1;
                    column += 1;
                    state = State::MultiLineStar;
                }
                '\n' => {
                    size += 1;
                    column = 1;
                    lines += 1;
                }
                _ => {
                    size += 1;
                    column += 1;
                }
            },
            State::MultiLineStar => match character {
                '/' => {
                    size += 1;
                    column += 1;
                    let comment = Comment::new(input[2..size - 2].to_owned());
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
    use crate::lexical::token::lexeme::comment::Comment;

    #[test]
    fn ok_single_line() {
        let input = "//mega ultra comment text\n";
        let expected = Ok((
            25,
            0,
            26,
            Comment::new("mega ultra comment text".to_owned()),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_multi_line() {
        let input = r#"/*
    This is the mega ultra test application!
*/"#;
        let expected = Ok((
            50,
            2,
            3,
            Comment::new("\n    This is the mega ultra test application!\n".to_owned()),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_single_line_unexpected_end() {
        let input = "//mega ultra comment text";
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_multi_line_unexpected_end() {
        let input = r#"/* This is the mega ultra test application!"#;
        let expected = Err(Error::UnexpectedEnd);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_not_a_comment() {
        let input = "not a comment text";
        let expected = Err(Error::NotAComment);
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_not_a_comment_one_slash() {
        let input = "/almost a comment text";
        let expected = Err(Error::NotAComment);
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
