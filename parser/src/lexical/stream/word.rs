//!
//! The lexical word parser.
//!
//! The result can be either of:
//! 1. An identifier
//! 2. A keyword
//! 3. A boolean literal
//! 4. An underscore symbol
//!

use std::convert::TryFrom;

use failure::Fail;

use crate::lexical::BooleanLiteral;
use crate::lexical::Identifier;
use crate::lexical::IdentifierError;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;

pub enum State {
    Start,
    Continue,
}

#[derive(Debug, Fail, Clone, PartialEq)]
pub enum Error {
    #[fail(display = "empty identifier")]
    EmptyIdentifier,
}

pub fn parse(input: &str) -> Result<(usize, Lexeme), Error> {
    let mut state = State::Start;
    let mut size = 0;

    while let Some(character) = input.chars().nth(size) {
        match state {
            State::Start => {
                if !Identifier::can_start_with(character) {
                    break;
                }
                state = State::Continue;
            }
            State::Continue => {
                if !Identifier::can_contain_since_index_1(character) {
                    break;
                }
            }
        }

        size += 1;
    }

    let lexeme = match Identifier::try_from(&input[..size]) {
        Ok(identifier) => Lexeme::Identifier(identifier),
        Err(IdentifierError::IsEmpty) => return Err(Error::EmptyIdentifier),
        Err(IdentifierError::IsUnderscore) => Lexeme::Symbol(Symbol::Underscore),
        Err(IdentifierError::IsKeyword(keyword)) => match BooleanLiteral::try_from(keyword) {
            Ok(boolean) => Lexeme::Literal(Literal::Boolean(boolean)),
            Err(keyword) => Lexeme::Keyword(keyword),
        },
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
    use crate::lexical::Symbol;

    #[test]
    fn ok_identifier() {
        let input = "xyz";
        let expected = Ok((
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_identifier_below_field_range() {
        let input = "u0";
        let expected = Ok((
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_identifier_above_field_range() {
        let input = "u256";
        let expected = Ok((
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_identifier_invalid_modulo() {
        let input = "u119";
        let expected = Ok((
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_keyword() {
        let input = "match";
        let expected = Ok((input.len(), Lexeme::Keyword(Keyword::Match)));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_keyword_signed_integer_min() {
        let input = "i8";
        let expected = Ok((input.len(), Lexeme::Keyword(Keyword::new_integer_signed(8))));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_keyword_unsigned_integer_max() {
        let input = "u248";
        let expected = Ok((
            input.len(),
            Lexeme::Keyword(Keyword::new_integer_unsigned(248)),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_literal_boolean() {
        let input = "true";
        let expected = Ok((
            input.len(),
            Lexeme::Literal(Literal::Boolean(BooleanLiteral::True)),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn ok_symbol_underscore() {
        let input = "_";
        let expected = Ok((input.len(), Lexeme::Symbol(Symbol::Underscore)));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn error_empty_identifier() {
        let input = "";
        let expected = Err(Error::EmptyIdentifier);
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
