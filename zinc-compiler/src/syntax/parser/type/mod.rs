//!
//! The type parser.
//!

mod array;
mod tuple;

use std::cell::RefCell;
use std::rc::Rc;

use self::array::Parser as ArrayTypeParser;
use self::tuple::Parser as TupleTypeParser;
use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::PathOperandParser;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;

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
        match crate::syntax::take_or_next(initial.take(), stream.clone())? {
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
                _ => Err(Error::Syntax(SyntaxError::Expected(
                    location,
                    vec!["{type}"],
                    Lexeme::Keyword(keyword),
                ))),
            },
            token @ Token {
                lexeme: Lexeme::Identifier(_),
                ..
            } => {
                let location = token.location;
                let (expression, next) = PathOperandParser::default().parse(stream, Some(token))?;
                self.builder.set_location(location);
                self.builder.set_path_expression(expression);
                Ok((self.builder.finish(), next))
            }
            token @ Token {
                lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                ..
            } => Ok((ArrayTypeParser::default().parse(stream, Some(token))?, None)),
            token @ Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => Ok((TupleTypeParser::default().parse(stream, Some(token))?, None)),
            Token {
                lexeme: Lexeme::Symbol(Symbol::Ampersand),
                location,
            } => {
                let (inner, next) = Self::default().parse(stream, None)?;
                self.builder.set_location(location);
                self.builder.set_reference_inner(inner);
                Ok((self.builder.finish(), next))
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec!["{type}", "{identifier}", "(", "["],
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
    use crate::error::Error;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Error as SyntaxError;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_unit() {
        let input = "()";

        let expected = Ok((
            Type::new(Location::new(1, 1), TypeVariant::new_unit()),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_integer() {
        let input = "u232";

        let expected = Ok((
            Type::new(Location::new(1, 1), TypeVariant::new_integer_unsigned(232)),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_field() {
        let input = "field";

        let expected = Ok((
            Type::new(Location::new(1, 1), TypeVariant::new_field()),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_array() {
        let input = "[field; 8]";

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::new_array(
                    TypeVariant::new_field(),
                    IntegerLiteral::new(
                        Location::new(1, 9),
                        lexical::IntegerLiteral::new_decimal("8".to_owned()),
                    ),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_array_double() {
        let input = "[[field; 8]; 8]";

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::new_array(
                    TypeVariant::new_array(
                        TypeVariant::new_field(),
                        IntegerLiteral::new(
                            Location::new(1, 10),
                            lexical::IntegerLiteral::new_decimal("8".to_owned()),
                        ),
                    ),
                    IntegerLiteral::new(
                        Location::new(1, 14),
                        lexical::IntegerLiteral::new_decimal("8".to_owned()),
                    ),
                ),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_tuple_single() {
        let input = "(field,)";

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::new_tuple(vec![TypeVariant::Field]),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_tuple_multiple() {
        let input = "(field, (), [u8; 4])";

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::new_tuple(vec![
                    TypeVariant::Field,
                    TypeVariant::Unit,
                    TypeVariant::new_array(
                        TypeVariant::new_integer_unsigned(8),
                        IntegerLiteral::new(
                            Location::new(1, 18),
                            lexical::IntegerLiteral::new_decimal("4".to_owned()),
                        ),
                    ),
                ]),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_tuple_nested() {
        let input = "((field, field),)";

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::new_tuple(vec![TypeVariant::new_tuple(vec![
                    TypeVariant::Field,
                    TypeVariant::Field,
                ])]),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_reference() {
        let input = "&field";

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::new_reference(TypeVariant::new_field()),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_double_reference() {
        let input = "&(&field)";

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::new_reference(TypeVariant::new_reference(TypeVariant::new_field())),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_alias_identifier() {
        let input = "MegaStructure";

        let expected = Ok((
            Type::new(
                Location::new(1, 1),
                TypeVariant::new_alias(Expression::new(
                    Location::new(1, 1),
                    vec![ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                            Location::new(1, 1),
                            "MegaStructure".to_owned(),
                        ))),
                    )],
                )),
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 14))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

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

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
