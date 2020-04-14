//!
//! The variant parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::literal::Literal as LexicalLiteral;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::variant::builder::Builder as VariantBuilder;
use crate::syntax::tree::variant::Variant;

static HINT_EXPECTED_IDENTIFIER: &str =
    "enumeration variant must have an identifier, e.g. `Value = 42`";
static HINT_EXPECTED_VALUE: &str = "enumeration variant must be initialized, e.g. `Value = 42`";

#[derive(Default)]
pub struct Parser {
    builder: VariantBuilder,
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
        mut initial: Option<Token>,
    ) -> Result<(Variant, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                let identifier = Identifier::new(location, identifier.inner);
                self.builder.set_location(location);
                self.builder.set_identifier(identifier);
            }
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::expected_identifier(
                    location,
                    lexeme,
                    Some(HINT_EXPECTED_IDENTIFIER),
                )));
            }
        }

        match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Equals),
                ..
            } => {}
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::expected_value(
                    location,
                    lexeme,
                    Some(HINT_EXPECTED_VALUE),
                )));
            }
        }

        match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
            Token {
                lexeme: Lexeme::Literal(LexicalLiteral::Integer(literal)),
                location,
            } => {
                self.builder
                    .set_literal(IntegerLiteral::new(location, literal));
                Ok((self.builder.finish(), self.next.take()))
            }
            Token { lexeme, location } => Err(Error::Syntax(
                SyntaxError::expected_integer_literal(location, lexeme),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::identifier::Identifier as LexicalIdentifier;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::variant::Variant;

    #[test]
    fn ok() {
        let input = r#"A = 1"#;

        let expected = Ok((
            Variant::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), "A".to_owned()),
                IntegerLiteral::new(
                    Location::new(1, 5),
                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_value() {
        let input = r#"A"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_value(
            Location::new(1, 2),
            Lexeme::Eof,
            Some(super::HINT_EXPECTED_VALUE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_integer_literal() {
        let input = r#"A = id"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_integer_literal(
            Location::new(1, 5),
            Lexeme::Identifier(LexicalIdentifier::new("id".to_owned())),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
