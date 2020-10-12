//!
//! The lexical string parser.
//!

use std::str;

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
/// The parser error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The lexeme is not a string, which means that another parser must be run.
    NotAString,
    /// The string has not been terminated, which ends up with an entire file treated as an unterminated string.
    UnterminatedDoubleQuote {
        /// The number of lines in the unterminated string.
        lines: usize,
        /// The column where the unterminated string ends.
        column: usize,
    },
}

///
/// Parses a string literal.
///
/// Example:
/// '"Zinc is the best language for ZKP"'
///
pub fn parse(input: &str) -> Result<(usize, String), Error> {
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
                    return Ok((size, value));
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

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;

    #[test]
    fn ok() {
        let input = r#""some string""#;
        let expected = Ok((input.len(), "some string".to_owned()));
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
}
