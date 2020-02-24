//!
//! The type parser.
//!

pub mod array;
pub mod path;
pub mod tuple;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::tree::r#type::builder::Builder as TypeBuilder;
use crate::syntax::tree::r#type::Type;

use self::array::Parser as ArrayParser;
use self::path::Parser as PathParser;
use self::tuple::Parser as TupleParser;

#[derive(Default)]
pub struct Parser {
    builder: TypeBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Type, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
            Token {
                lexeme: Lexeme::Keyword(keyword),
                location,
            } => match keyword {
                keyword @ Keyword::Bool
                | keyword @ Keyword::IntegerSigned { .. }
                | keyword @ Keyword::IntegerUnsigned { .. }
                | keyword @ Keyword::Field => {
                    self.builder.set_location(location);
                    self.builder.set_keyword(keyword);
                    Ok((self.builder.finish(), None))
                }
                _ => Err(Error::Syntax(SyntaxError::expected_type(
                    location,
                    Lexeme::Keyword(keyword),
                    None,
                ))),
            },
            token
            @
            Token {
                lexeme: Lexeme::Identifier(_),
                ..
            } => {
                let location = token.location;
                let (expression, next) = PathParser::default().parse(stream, Some(token))?;
                self.builder.set_location(location);
                self.builder.set_path_expression(expression);
                Ok((self.builder.finish(), next))
            }
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                ..
            } => Ok((ArrayParser::default().parse(stream, Some(token))?, None)),
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => Ok((TupleParser::default().parse(stream, Some(token))?, None)),
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::expected_type(
                location, lexeme, None,
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::error::Error;
    use crate::lexical;
    use crate::lexical::Keyword;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
    use crate::syntax::tree::r#type::Type;

    #[test]
    fn ok_bool() {
        let input = "bool";

        let expected = Ok((Type::new(Location::new(1, 1), TypeVariant::boolean()), None));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_integer() {
        let input = "u232";

        let expected = Ok((
            Type::new(Location::new(1, 1), TypeVariant::integer_unsigned(232)),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_field() {
        let input = "field";

        let expected = Ok((Type::new(Location::new(1, 1), TypeVariant::field()), None));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type_keyword() {
        let input = "while";

        let expected = Err(Error::Syntax(SyntaxError::expected_type(
            Location::new(1, 1),
            Lexeme::Keyword(Keyword::While),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_type() {
        let input = "42";

        let expected = Err(Error::Syntax(SyntaxError::expected_type(
            Location::new(1, 1),
            Lexeme::Literal(lexical::Literal::Integer(
                lexical::IntegerLiteral::new_decimal("42".to_owned()),
            )),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
