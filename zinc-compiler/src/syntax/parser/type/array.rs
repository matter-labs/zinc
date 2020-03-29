//!
//! The array type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::Parser as ExpressionParser;
use crate::syntax::parser::r#type::Parser as TypeParser;
use crate::syntax::tree::r#type::builder::Builder as TypeBuilder;
use crate::syntax::tree::r#type::Type;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BracketSquareLeft,
    Type,
    Semicolon,
    SizeExpression,
    BracketSquareRight,
}

impl Default for State {
    fn default() -> Self {
        State::BracketSquareLeft
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    next: Option<Token>,
    builder: TypeBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<Type, Error> {
        loop {
            match self.state {
                State::BracketSquareLeft => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Type;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["["],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
                State::Type => {
                    let (array_type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_array_type_variant(array_type.variant);
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => {
                            self.state = State::SizeExpression;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![";"],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
                State::SizeExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.set_array_size_expression(expression);
                    self.state = State::BracketSquareRight;
                }
                State::BracketSquareRight => {
                    return match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec!["]"],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::cell::RefCell;
//     use std::rc::Rc;
//
//     use super::Parser;
//     use crate::error::Error;
//     use crate::lexical;
//     use crate::lexical::Lexeme;
//     use crate::lexical::Location;
//     use crate::lexical::Symbol;
//     use crate::lexical::TokenStream;
//     use crate::syntax::error::Error as SyntaxError;
//     use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
//     use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
//     use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
//     use crate::syntax::tree::r#type::variant::Variant as TypeVariant;
//     use crate::syntax::tree::r#type::Type;
//
//     #[test]
//     fn ok() {
//         let input = "[field; 8]";
//
//         let expected = Ok(Type::new(
//             Location::new(1, 1),
//             TypeVariant::array(
//                 TypeVariant::field(),
//                 Expression::new(
//                     Location::new(1, 9),
//                     vec![ExpressionElement::new(
//                         Location::new(1, 9),
//                         ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                             IntegerLiteral::new(
//                                 Location::new(1, 9),
//                                 lexical::IntegerLiteral::new_decimal("8".to_owned()),
//                             ),
//                         )),
//                     )],
//                 ),
//             ),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_size_expression() {
//         let input = "[field; 4 * 4]";
//
//         let expected = Ok(Type::new(
//             Location::new(1, 1),
//             TypeVariant::array(
//                 TypeVariant::field(),
//                 Expression::new(
//                     Location::new(1, 9),
//                     vec![
//                         ExpressionElement::new(
//                             Location::new(1, 9),
//                             ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                                 IntegerLiteral::new(
//                                     Location::new(1, 9),
//                                     lexical::IntegerLiteral::new_decimal("4".to_owned()),
//                                 ),
//                             )),
//                         ),
//                         ExpressionElement::new(
//                             Location::new(1, 13),
//                             ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                                 IntegerLiteral::new(
//                                     Location::new(1, 13),
//                                     lexical::IntegerLiteral::new_decimal("4".to_owned()),
//                                 ),
//                             )),
//                         ),
//                         ExpressionElement::new(
//                             Location::new(1, 11),
//                             ExpressionObject::Operator(ExpressionOperator::Multiplication),
//                         ),
//                     ],
//                 ),
//             ),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_nested() {
//         let input = "[[field; 8]; 8]";
//
//         let expected = Ok(Type::new(
//             Location::new(1, 1),
//             TypeVariant::array(
//                 TypeVariant::array(
//                     TypeVariant::field(),
//                     Expression::new(
//                         Location::new(1, 10),
//                         vec![ExpressionElement::new(
//                             Location::new(1, 10),
//                             ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                                 IntegerLiteral::new(
//                                     Location::new(1, 10),
//                                     lexical::IntegerLiteral::new_decimal("8".to_owned()),
//                                 ),
//                             )),
//                         )],
//                     ),
//                 ),
//                 Expression::new(
//                     Location::new(1, 14),
//                     vec![ExpressionElement::new(
//                         Location::new(1, 14),
//                         ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                             IntegerLiteral::new(
//                                 Location::new(1, 14),
//                                 lexical::IntegerLiteral::new_decimal("8".to_owned()),
//                             ),
//                         )),
//                     )],
//                 ),
//             ),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn error_expected_semicolon() {
//         let input = "[field, 8]";
//
//         let expected = Err(Error::Syntax(SyntaxError::expected_one_of(
//             Location::new(1, 7),
//             vec![";"],
//             Lexeme::Symbol(Symbol::Comma),
//             None,
//         )));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn error_expected_bracket_square_right() {
//         let input = "[field; 8)";
//
//         let expected = Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
//             Location::new(1, 10),
//             vec!["]"],
//             Lexeme::Symbol(Symbol::ParenthesisRight),
//             None,
//         )));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
// }
