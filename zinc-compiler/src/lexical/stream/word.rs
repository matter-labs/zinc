//!
//! The lexical word parser.
//!

use std::convert::TryFrom;

use crate::lexical::token::lexeme::identifier::Error as IdentifierError;
use crate::lexical::token::lexeme::identifier::Identifier;
use crate::lexical::token::lexeme::literal::boolean::Boolean;
use crate::lexical::token::lexeme::literal::Literal;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;

pub enum State {
    Start,
    Continue,
}

pub fn parse(input: &str) -> (usize, Lexeme) {
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
                if !Identifier::can_contain_after_start(character) {
                    break;
                }
            }
        }

        size += 1;
    }

    // The result can be either of:
    // 1. An identifier
    // 2. A keyword
    // 3. A boolean literal
    // 4. An underscore symbol

    let lexeme = match Identifier::try_from(&input[..size]) {
        Ok(identifier) => Lexeme::Identifier(identifier),
        Err(IdentifierError::IsUnderscore) => Lexeme::Symbol(Symbol::Underscore),
        Err(IdentifierError::IsKeyword(keyword)) => match Boolean::try_from(keyword) {
            Ok(boolean) => Lexeme::Literal(Literal::Boolean(boolean)),
            Err(keyword) => Lexeme::Keyword(keyword),
        },
    };
    (size, lexeme)
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::lexical::token::lexeme::identifier::Identifier;
    use crate::lexical::token::lexeme::keyword::Keyword;
    use crate::lexical::token::lexeme::literal::boolean::Boolean;
    use crate::lexical::token::lexeme::literal::Literal;
    use crate::lexical::token::lexeme::symbol::Symbol;
    use crate::lexical::token::lexeme::Lexeme;

    #[test]
    fn ok_identifier() {
        let input = "xyz";
        let expected = (
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        );
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_identifier_below_field_range() {
        let input = "u0";
        let expected = (
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        );
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_identifier_above_field_range() {
        let input = "u256";
        let expected = (
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        );
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_identifier_invalid_modulo() {
        let input = "u119";
        let expected = (
            input.len(),
            Lexeme::Identifier(Identifier::new(input.to_owned())),
        );
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_keyword() {
        let input = "match";
        let expected = (input.len(), Lexeme::Keyword(Keyword::Match));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_keyword_signed_integer_min() {
        let input = "i8";
        let expected = (input.len(), Lexeme::Keyword(Keyword::new_integer_signed(8)));
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_keyword_unsigned_integer_max() {
        let input = "u248";
        let expected = (
            input.len(),
            Lexeme::Keyword(Keyword::new_integer_unsigned(crate::BITLENGTH_MAX_INT)),
        );
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_literal_boolean() {
        let input = "true";
        let expected = (
            input.len(),
            Lexeme::Literal(Literal::Boolean(Boolean::True)),
        );
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_symbol_underscore() {
        let input = "_";
        let expected = (input.len(), Lexeme::Symbol(Symbol::Underscore));
        let result = parse(input);
        assert_eq!(result, expected);
    }
}
