//!
//! The type parser.
//!

mod array;
mod tuple;

use std::cell::RefCell;
use std::rc::Rc;

use self::array::Parser as ArrayTypeParser;
use self::tuple::Parser as TupleTypeParser;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;
use crate::Error;

#[derive(Default)]
pub struct Parser {
    builder: TypeBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Type, Error> {
        let peek = stream.borrow_mut().peek();
        match peek {
            Some(Ok(Token {
                lexeme: Lexeme::Keyword(keyword),
                location,
            })) => match keyword {
                keyword @ Keyword::Bool
                | keyword @ Keyword::I { .. }
                | keyword @ Keyword::U { .. }
                | keyword @ Keyword::Field => {
                    stream.borrow_mut().next();
                    self.builder.set_location(location);
                    self.builder.set_keyword(keyword);
                    Ok(self.builder.finish())
                }
                _ => Err(Error::Syntax(SyntaxError::Expected(
                    location,
                    vec!["{type}"],
                    Lexeme::Keyword(keyword),
                ))),
            },
            Some(Ok(Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            })) => {
                stream.borrow_mut().next();
                self.builder.set_location(location);
                self.builder.set_alias_identifier(identifier.name);
                Ok(self.builder.finish())
            }
            Some(Ok(Token {
                lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                ..
            })) => ArrayTypeParser::default().parse(stream.clone()),
            Some(Ok(Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            })) => TupleTypeParser::default().parse(stream.clone()),
            Some(Ok(Token { lexeme, location })) => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec!["{type}", "{identifier}", "(", "["],
                lexeme,
            ))),
            Some(Err(error)) => Err(Error::Lexical(error)),
            None => Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                stream.borrow().location(),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::Error as SyntaxError;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;
    use crate::Error;

    #[test]
    fn ok_unit() {
        let input = "()";

        let expected = Ok(Type::new(Location::new(1, 1), TypeVariant::new_unit()));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_integer() {
        let input = "u232";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::new_integer_unsigned(232),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_field() {
        let input = "field";

        let expected = Ok(Type::new(Location::new(1, 1), TypeVariant::new_field()));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_array() {
        let input = "[field; 8]";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::new_array(TypeVariant::new_field(), 8),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_array_double() {
        let input = "[[field; 8]; 8]";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::new_array(TypeVariant::new_array(TypeVariant::new_field(), 8), 8),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_tuple_single() {
        let input = "(field,)";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::new_tuple(vec![TypeVariant::Field]),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_tuple_multiple() {
        let input = "(field, (), [u8; 4])";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::new_tuple(vec![
                TypeVariant::Field,
                TypeVariant::Unit,
                TypeVariant::new_array(TypeVariant::new_integer_unsigned(8), 4),
            ]),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_tuple_nested() {
        let input = "((field, field),)";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::new_tuple(vec![TypeVariant::new_tuple(vec![
                TypeVariant::Field,
                TypeVariant::Field,
            ])]),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_alias_identifier() {
        let input = "MegaStructure";

        let expected = Ok(Type::new(
            Location::new(1, 1),
            TypeVariant::new_alias(input.to_owned()),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn err_array_expected_semicolon() {
        let input = "[field, 8]";

        let expected = Err(Error::Syntax(SyntaxError::Expected(
            Location::new(1, 7),
            vec![";"],
            Lexeme::Symbol(Symbol::Comma),
        )));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
