//!
//! The field parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::r#type::Parser as TypeParser;
use crate::tree::field::builder::Builder as FieldBuilder;
use crate::tree::field::Field;
use crate::tree::identifier::Identifier;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str = "structure field must have an identifier, e.g. `a: u8`";
/// The missing type error hint.
pub static HINT_EXPECTED_TYPE: &str = "structure field must have a type, e.g. `a: u8`";

///
/// The field parser.
///
#[derive(Default)]
pub struct Parser {
    /// The builder of the parsed value.
    builder: FieldBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a structure field.
    ///
    /// 'a: u8'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(Field, Option<Token>), ParsingError> {
        self.next = initial;

        match crate::parser::take_or_next(self.next.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                let identifier = Identifier::new(location, identifier.inner);
                self.builder.set_location(location);
                self.builder.set_identifier(identifier);
            }
            Token { lexeme, location } => {
                return Err(ParsingError::Syntax(SyntaxError::expected_identifier(
                    location,
                    lexeme,
                    Some(HINT_EXPECTED_IDENTIFIER),
                )));
            }
        }

        match crate::parser::take_or_next(self.next.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Colon),
                ..
            } => {}
            Token { lexeme, location } => {
                return Err(ParsingError::Syntax(SyntaxError::expected_type(
                    location,
                    lexeme,
                    Some(HINT_EXPECTED_TYPE),
                )));
            }
        }

        let (r#type, next) = TypeParser::default().parse(stream, None)?;
        self.builder.set_type(r#type);
        Ok((self.builder.finish(), next))
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::field::Field;
    use crate::tree::identifier::Identifier;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok() {
        let input = r#"id: u232"#;

        let expected = Ok((
            Field::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 1), "id".to_owned()),
                Type::new(Location::test(1, 5), TypeVariant::integer_unsigned(232)),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"id"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_type(
            Location::test(1, 3),
            Lexeme::Eof,
            Some(crate::parser::field::HINT_EXPECTED_TYPE),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
