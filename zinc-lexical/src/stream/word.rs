//!
//! The lexical word parser.
//!

use std::convert::TryFrom;
use std::str::FromStr;

use crate::token::lexeme::identifier::Error as IdentifierError;
use crate::token::lexeme::identifier::Identifier;
use crate::token::lexeme::literal::boolean::Boolean;
use crate::token::lexeme::literal::Literal;
use crate::token::lexeme::symbol::Symbol;
use crate::token::lexeme::Lexeme;

///
/// The parser state.
///
pub enum State {
    /// The initial state.
    Start,
    /// The first character has been parsed so far.
    Continue,
}

///
/// Parses a word. The word can result into several token types:
///
/// 1. An identifier
/// 'value'
/// Any valid identifier which is not a keyword.
///
/// 2. An underscore symbol
/// '_'
/// The symbol can potentially start an identifier, but if there is no alpha symbol after the
/// underscore, it is not a valid identifier, so the underscore is treated as a symbol token.
///
/// 3. A boolean literal
/// 'true'
/// The literal is also a keyword, but is was decided to treat literals as a separate token type.
///
/// 4. A keyword
/// 'for'
/// Any keyword which is not a boolean literal.
///
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

    let lexeme = match Identifier::from_str(&input[..size]) {
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
    use crate::token::lexeme::identifier::Identifier;
    use crate::token::lexeme::keyword::Keyword;
    use crate::token::lexeme::literal::boolean::Boolean;
    use crate::token::lexeme::literal::Literal;
    use crate::token::lexeme::symbol::Symbol;
    use crate::token::lexeme::Lexeme;

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
            Lexeme::Keyword(Keyword::new_integer_unsigned(
                zinc_const::bitlength::INTEGER_MAX,
            )),
        );
        let result = parse(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn ok_literal_boolean() {
        let input = "true";
        let expected = (
            input.len(),
            Lexeme::Literal(Literal::Boolean(Boolean::r#true())),
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
