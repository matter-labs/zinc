//!
//! The lexical comment parser.
//!

use crate::lexical::token::lexeme::comment::Comment;

pub enum State {
    Start,
    Slash,
    SingleLine,
    MultiLine,
    MultiLineStar,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    NotAComment,
    UnterminatedBlock { lines: usize, column: usize },
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
pub fn parse(input: &str) -> Result<(usize, usize, usize, Comment), Error> {
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
                    let comment = Comment::new(input[2..size - 1].to_owned());
                    return Ok((size, lines, column, comment));
                }
                Some(_) => {
                    size += 1;
                    column += 1;
                }
                None => {
                    let comment = Comment::new(input[2..size].to_owned());
                    return Ok((size, lines, column, comment));
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
                    let comment = Comment::new(input[2..size - 2].to_owned());
                    return Ok((size, lines, column, comment));
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

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;
    use crate::lexical::token::lexeme::comment::Comment;

    #[test]
    fn ok_line_with_break() {
        let input = r#"//mega ultra comment text
"#;
        let expected = Ok((
            input.len(),
            input.lines().count(),
            input.len() + 1,
            Comment::new("mega ultra comment text".to_owned()),
        ));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_line_with_eof() {
        let input = r#"//mega ultra comment text"#;
        let expected = Ok((
            input.len(),
            input.lines().count() - 1,
            input.len() + 1,
            Comment::new("mega ultra comment text".to_owned()),
        ));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_block_one_line() {
        let input = r#"/*This is the mega ultra test application!*/"#;
        let expected = Ok((
            input.len(),
            input.lines().count() - 1,
            input.len() + 1,
            Comment::new("This is the mega ultra test application!".to_owned()),
        ));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_block_multi_line() {
        let input = r#"/*
    This is the mega ultra test application!
*/"#;
        let expected = Ok((
            input.len(),
            input.lines().count() - 1,
            input.lines().last().unwrap_or("").len() + 1,
            Comment::new("\n    This is the mega ultra test application!\n".to_owned()),
        ));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_not_a_comment() {
        let input = r#"not a comment text"#;
        let expected = Err(Error::NotAComment);
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_not_a_comment_one_slash() {
        let input = r#"/almost a comment text"#;
        let expected = Err(Error::NotAComment);
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn error_unterminated_block() {
        let input = r#"/* unterminated"#;
        let expected = Err(Error::UnterminatedBlock {
            lines: input.lines().count() - 1,
            column: input.len() + 1,
        });
        let result = parse(input);
        assert_eq!(result, expected);
    }
}
