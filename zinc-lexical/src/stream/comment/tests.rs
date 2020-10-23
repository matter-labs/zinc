//!
//! The lexical comment parser tests.
//!

use super::parse;
use super::Error;
use super::Output;
use crate::token::lexeme::comment::Comment;

#[test]
fn ok_line_with_break() {
    let input = r#"//mega ultra comment text
"#;
    let expected = Ok(Output::new(
        input.len(),
        input.lines().count(),
        input.len() + 1,
        Comment::new_line("mega ultra comment text".to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_line_with_eof() {
    let input = r#"//mega ultra comment text"#;
    let expected = Ok(Output::new(
        input.len(),
        input.lines().count() - 1,
        input.len() + 1,
        Comment::new_line("mega ultra comment text".to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_block_one_line() {
    let input = r#"/*This is the mega ultra test application!*/"#;
    let expected = Ok(Output::new(
        input.len(),
        input.lines().count() - 1,
        input.len() + 1,
        Comment::new_block("This is the mega ultra test application!".to_owned()),
    ));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn ok_block_multi_line() {
    let input = r#"/*
    This is the mega ultra test application!
*/"#;
    let expected = Ok(Output::new(
        input.len(),
        input.lines().count() - 1,
        input.lines().last().unwrap_or("").len() + 1,
        Comment::new_block("\n    This is the mega ultra test application!\n".to_owned()),
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
