//!
//! The word parser.
//!
//! The word can be a keyword, identifier, or boolean literal.
//!

use std::convert::TryFrom;

use failure::Fail;
use serde_derive::Serialize;

use crate::lexical::BooleanLiteral;
use crate::lexical::Identifier;
use crate::lexical::IdentifierError;
use crate::lexical::Lexeme;
use crate::lexical::Literal;

pub enum State {
    Start,
    Continue,
}

#[derive(Debug, Fail, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    #[fail(display = "empty identifier")]
    EmptyIdentifier,
}

pub fn parse(bytes: &[u8]) -> Result<(usize, Lexeme), Error> {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(byte) = bytes.get(size).copied() {
        match state {
            State::Start => {
                if !Identifier::can_start_with(byte) {
                    break;
                }
                state = State::Continue;
            }
            State::Continue => {
                if !Identifier::can_contain_since_index_1(byte) {
                    break;
                }
            }
        }

        size += 1;
    }

    let lexeme = match Identifier::try_from(&bytes[..size]) {
        Ok(identifier) => Lexeme::Identifier(identifier),
        Err(IdentifierError::IsKeyword(keyword)) => match BooleanLiteral::try_from(keyword) {
            Ok(boolean) => Lexeme::Literal(Literal::Boolean(boolean)),
            Err(keyword) => Lexeme::Keyword(keyword),
        },
        Err(IdentifierError::IsEmpty) => return Err(Error::EmptyIdentifier),
    };
    Ok((size, lexeme))
}

#[cfg(test)]
mod tests {
    use super::parse;
    use super::Error;
    use crate::lexical::BooleanLiteral;
    use crate::lexical::Identifier;
    use crate::lexical::Keyword;
    use crate::lexical::Lexeme;
    use crate::lexical::Literal;

    #[test]
    fn identifier_ok() {
        let input = b"xyz";
        let expected = Ok((3, Lexeme::Identifier(Identifier::new(input))));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn identifier_below_field_range_ok() {
        let input = b"uint0";
        let expected = Ok((5, Lexeme::Identifier(Identifier::new(input))));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn identifier_above_field_range_ok() {
        let input = b"uint254";
        let expected = Ok((7, Lexeme::Identifier(Identifier::new(input))));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn keyword_ok() {
        let input = b"require";
        let expected = Ok((7, Lexeme::Keyword(Keyword::Require)));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn keyword_int_min_ok() {
        let input = b"int1";
        let expected = Ok((4, Lexeme::Keyword(Keyword::int(1))));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn keyword_uint_max_ok() {
        let input = b"uint253";
        let expected = Ok((7, Lexeme::Keyword(Keyword::uint(253))));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn literal_boolean_ok() {
        let input = b"true";
        let expected = Ok((4, Lexeme::Literal(Literal::Boolean(BooleanLiteral::True))));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_empty_identifier() {
        let input = b"";
        let expected = Err(Error::EmptyIdentifier);
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
