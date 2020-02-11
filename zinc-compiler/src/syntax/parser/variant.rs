//!
//! The variant parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::Variant;
use crate::syntax::VariantBuilder;

#[derive(Default)]
pub struct Parser {
    builder: VariantBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<Variant, Error> {
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
                lexeme: Lexeme::Symbol(Symbol::Equals),
                ..
            } => {}
            Token { lexeme, location } => {
                return Err(Error::Syntax(SyntaxError::Expected(
                    location,
                    vec!["="],
                    lexeme,
                )));
            }
        }

        match crate::syntax::take_or_next(self.next.take(), stream)? {
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::Integer(literal)),
                location,
            } => {
                self.builder
                    .set_literal(IntegerLiteral::new(location, literal));
                Ok(self.builder.finish())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec!["{integer}"],
                lexeme,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::Variant;

    #[test]
    fn ok_single() {
        let input = "A = 1";

        let expected = Ok(Variant::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 1), "A".to_owned()),
            IntegerLiteral::new(
                Location::new(1, 5),
                lexical::IntegerLiteral::new_decimal("1".to_owned()),
            ),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(expected, result);
    }
}
