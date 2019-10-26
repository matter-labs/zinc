//!
//! The variant parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Identifier;
use crate::syntax::Variant;
use crate::syntax::VariantBuilder;
use crate::Error;

#[derive(Default)]
pub struct Parser {
    builder: VariantBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<Variant, Error> {
        match match initial.take() {
            Some(token) => token,
            None => stream.borrow_mut().next()?,
        } {
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

        let next = stream.borrow_mut().next()?;
        match next {
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

        let next = stream.borrow_mut().next()?;
        match next {
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::Integer(literal)),
                ..
            } => {
                self.builder.set_literal(literal);
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
    use crate::lexical::IntegerLiteral;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Identifier;
    use crate::syntax::Variant;

    #[test]
    fn ok_single() {
        let input = "A = 1";

        let expected = Ok(Variant::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 1), "A".to_owned()),
            IntegerLiteral::new_decimal("1".to_owned()),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
