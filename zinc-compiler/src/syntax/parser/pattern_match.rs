//!
//! The match pattern parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BooleanLiteral;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchPattern;
use crate::syntax::MatchPatternBuilder;

#[derive(Default)]
pub struct Parser {
    builder: MatchPatternBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<MatchPattern, Error> {
        match match initial.take() {
            Some(token) => token,
            None => stream.borrow_mut().next()?,
        } {
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::Boolean(boolean)),
                location,
            } => {
                self.builder.set_location(location);
                self.builder
                    .set_boolean_literal(BooleanLiteral::new(location, boolean));
                Ok(self.builder.finish())
            }
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::Integer(integer)),
                location,
            } => {
                self.builder.set_location(location);
                self.builder
                    .set_integer_literal(IntegerLiteral::new(location, integer));
                Ok(self.builder.finish())
            }
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                self.builder.set_location(location);
                self.builder
                    .set_binding(Identifier::new(location, identifier.name));
                Ok(self.builder.finish())
            }
            Token {
                lexeme: Lexeme::Symbol(Symbol::Underscore),
                location,
            } => {
                self.builder.set_location(location);
                self.builder.set_ignoring();
                Ok(self.builder.finish())
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec!["{integer}", "{identifier}", "_"],
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
    use crate::syntax::BooleanLiteral;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::MatchPattern;
    use crate::syntax::MatchPatternVariant;

    #[test]
    fn ok_literal_boolean() {
        let input = "true";

        let expected = Ok(MatchPattern::new(
            Location::new(1, 1),
            MatchPatternVariant::BooleanLiteral(BooleanLiteral::new(
                Location::new(1, 1),
                lexical::BooleanLiteral::True,
            )),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_literal_integer() {
        let input = "42";

        let expected = Ok(MatchPattern::new(
            Location::new(1, 1),
            MatchPatternVariant::IntegerLiteral(IntegerLiteral::new(
                Location::new(1, 1),
                lexical::IntegerLiteral::new_decimal("42".to_owned()),
            )),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_binding() {
        let input = "value";

        let expected = Ok(MatchPattern::new(
            Location::new(1, 1),
            MatchPatternVariant::Binding(Identifier::new(Location::new(1, 1), "value".to_owned())),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_ignoring() {
        let input = "_";

        let expected = Ok(MatchPattern::new(
            Location::new(1, 1),
            MatchPatternVariant::Ignoring,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
