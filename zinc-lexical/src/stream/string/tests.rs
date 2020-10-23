//!
//! The lexical string literal parser tests.
//!

use super::parse;
use super::Error;
use super::Output;

#[test]
fn ok() {
    let input = r#""some string""#;
    let expected = Ok(Output::new(input.len(), "some string".to_owned()));
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn error_not_a_string() {
    let input = r#"no double quote here"#;
    let expected = Err(Error::NotAString);
    let result = parse(input);
    assert_eq!(result, expected);
}

#[test]
fn error_unterminated_double_quote() {
    let input = r#""some string"#;
    let expected = Err(Error::UnterminatedDoubleQuote {
        lines: input.lines().count() - 1,
        column: input.len() + 1,
    });
    let result = parse(input);
    assert_eq!(result, expected);
}
