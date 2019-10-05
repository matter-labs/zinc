//!
//! The word parser.
//!
//! The word can be a keyword, identifier, or boolean literal.
//!

use std::convert::TryFrom;

use failure::Fail;

use crate::lexical::BooleanLiteral;
use crate::lexical::Identifier;
use crate::lexical::IdentifierError;
use crate::lexical::Lexeme;
use crate::lexical::Literal;

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
        let input = "xyz";
        let expected = Ok((
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn identifier_below_field_range_ok() {
        let input = "u0";
        let expected = Ok((
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn identifier_above_field_range_ok() {
        let input = "u256";
        let expected = Ok((
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn identifier_invalid_modulo_ok() {
        let input = "u119";
        let expected = Ok((
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn keyword_ok() {
        let input = "require";
        let expected = Ok((input.len(), Lexeme::Keyword(Keyword::Require)));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn keyword_signed_integer_min_ok() {
        let input = "i8";
        let expected = Ok((input.len(), Lexeme::Keyword(Keyword::new_integer_signed(8))));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn keyword_unsigned_integer_max_ok() {
        let input = "u248";
        let expected = Ok((
            input.len(),
            Lexeme::Keyword(Keyword::new_integer_unsigned(248)),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn literal_boolean_ok() {
        let input = "true";
        let expected = Ok((
            input.len(),
            Lexeme::Literal(Literal::Boolean(BooleanLiteral::True)),
        ));
        let result = parse(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn err_empty_identifier() {
        let input = "";
        let expected = Err(Error::EmptyIdentifier);
        let result = parse(input);
        assert_eq!(expected, result);
    }
}
