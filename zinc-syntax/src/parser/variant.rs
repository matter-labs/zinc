//!
//! The variant parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Literal as LexicalLiteral;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::tree::identifier::Identifier;
use crate::tree::literal::integer::Literal as IntegerLiteral;
use crate::tree::variant::builder::Builder as VariantBuilder;
use crate::tree::variant::Variant;

/// The missing identifier error hint.
pub static HINT_EXPECTED_IDENTIFIER: &str =
    "enumeration variant must have an identifier, e.g. `Value = 42`";
/// The missing value error hint.
pub static HINT_EXPECTED_VALUE: &str = "enumeration variant must be initialized, e.g. `Value = 42`";

///
/// The variant parser.
///
#[derive(Default)]
pub struct Parser {
    /// The builder of the parsed value.
    builder: VariantBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an enum variant.
    ///
    /// 'A = 1'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(Variant, Option<Token>), ParsingError> {
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
                lexeme: Lexeme::Symbol(Symbol::Equals),
                ..
            } => {}
            Token { lexeme, location } => {
                return Err(ParsingError::Syntax(SyntaxError::expected_value(
                    location,
                    lexeme,
                    Some(HINT_EXPECTED_VALUE),
                )));
            }
        }

        match crate::parser::take_or_next(self.next.take(), stream)? {
            Token {
                lexeme: Lexeme::Literal(LexicalLiteral::Integer(literal)),
                location,
            } => {
                self.builder
                    .set_literal(IntegerLiteral::new(location, literal));
                Ok((self.builder.finish(), self.next.take()))
            }
            Token { lexeme, location } => Err(ParsingError::Syntax(
                SyntaxError::expected_integer_literal(location, lexeme),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::Identifier as LexicalIdentifier;
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::variant::Variant;

    #[test]
    fn ok() {
        let input = r#"A = 1"#;

        let expected = Ok((
            Variant::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 1), "A".to_owned()),
                IntegerLiteral::new(
                    Location::test(1, 5),
                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_value() {
        let input = r#"A"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_value(
            Location::test(1, 2),
            Lexeme::Eof,
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_integer_literal() {
        let input = r#"A = id"#;

        let expected = Err(ParsingError::Syntax(SyntaxError::expected_integer_literal(
            Location::test(1, 5),
            Lexeme::Identifier(LexicalIdentifier::new("id".to_owned())),
        )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
