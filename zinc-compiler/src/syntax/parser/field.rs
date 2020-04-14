//!
//! The field parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::r#type::Parser as TypeParser;
use crate::syntax::tree::field::builder::Builder as FieldBuilder;
use crate::syntax::tree::field::Field;
use crate::syntax::tree::identifier::Identifier;

static HINT_EXPECTED_IDENTIFIER: &str = "structure field must have an identifier, e.g. `a: u8`";
static HINT_EXPECTED_TYPE: &str = "structure field must have a type, e.g. `a: u8`";

#[derive(Default)]
pub struct Parser {
    builder: FieldBuilder,
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
        mut initial: Option<Token>,
    ) -> Result<(Field, Option<Token>), Error> {
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
                lexeme: Lexeme::Symbol(Symbol::Colon),
                ..
            } => {}
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::expected_type(
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
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::parser::field::HINT_EXPECTED_TYPE;
    use crate::syntax::tree::field::Field;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;

    #[test]
    fn ok() {
        let input = r#"id: u232"#;

        let expected = Ok((
            Field::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), "id".to_owned()),
                Type::new(Location::new(1, 5), TypeVariant::integer_unsigned(232)),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = r#"id"#;

        let expected = Err(Error::Syntax(SyntaxError::expected_type(
            Location::new(1, 3),
            Lexeme::Eof,
            Some(HINT_EXPECTED_TYPE),
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
