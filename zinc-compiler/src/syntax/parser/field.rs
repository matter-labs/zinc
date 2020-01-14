//!
//! The field parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Field;
use crate::syntax::FieldBuilder;
use crate::syntax::Identifier;
use crate::syntax::TypeParser;

#[derive(Default)]
pub struct Parser {
    builder: FieldBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Field, Option<Token>), Error> {
        match crate::syntax::take_or_next(initial.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                let identifier = Identifier::new(location, identifier.name);
                self.builder.set_location(location);
                self.builder.set_identifier(identifier);
            }
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::Expected(
                    location,
                    vec!["{identifier}"],
                    lexeme,
                )));
            }
        }

        match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Symbol(Symbol::Colon),
                ..
            } => {}
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::Expected(
                    location,
                    vec![":"],
                    lexeme,
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
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Field;
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_single() {
        let input = "a: u232";

        let expected = Ok((
            Field::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), "a".to_owned()),
                Type::new(Location::new(1, 4), TypeVariant::new_integer_unsigned(232)),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
