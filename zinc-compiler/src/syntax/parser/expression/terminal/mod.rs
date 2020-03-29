//!
//! The terminal operand parser.
//!

pub mod array;
pub mod block;
pub mod conditional;
pub mod list;
pub mod r#match;
pub mod structure;
pub mod tuple;

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::syntax::tree::identifier::builder::Builder as IdentifierBuilder;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::literal::string::Literal as StringLiteral;

use self::array::Parser as ArrayExpressionParser;
use self::block::Parser as BlockExpressionParser;
use self::conditional::Parser as ConditionalExpressionParser;
use self::r#match::Parser as MatchExpressionParser;
use self::structure::Parser as StructureExpressionParser;
use self::tuple::Parser as TupleExpressionParser;

#[derive(Default)]
pub struct Parser {}

impl Parser {
    pub fn parse(
        self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ExpressionOperand, Location, Option<Token>), Error> {
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => {
                let location = token.location;
                TupleExpressionParser::default()
                    .parse(stream, Some(token))
                    .map(|(operand, token)| (operand, location, token))
            }
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                ..
            } => {
                let location = token.location;
                BlockExpressionParser::default()
                    .parse(stream, Some(token))
                    .map(|(operand, token)| (ExpressionOperand::Block(operand), location, token))
            }
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                ..
            } => {
                let location = token.location;
                ArrayExpressionParser::default()
                    .parse(stream, Some(token))
                    .map(|(operand, token)| (ExpressionOperand::Array(operand), location, token))
            }
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::If),
                ..
            } => {
                let location = token.location;
                ConditionalExpressionParser::default()
                    .parse(stream, Some(token))
                    .map(|(operand, token)| {
                        (ExpressionOperand::Conditional(operand), location, token)
                    })
            }
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Match),
                ..
            } => {
                let location = token.location;
                MatchExpressionParser::default()
                    .parse(stream, Some(token))
                    .map(|(operand, token)| (ExpressionOperand::Match(operand), location, token))
            }
            token
            @
            Token {
                lexeme: Lexeme::Identifier(..),
                ..
            } => {
                let location = token.location;
                let (expression, next) =
                    StructureExpressionParser::default().parse(stream, Some(token))?;
                if expression.is_struct {
                    Ok((ExpressionOperand::Structure(expression), location, next))
                } else {
                    let mut builder = IdentifierBuilder::default();
                    builder.set_location(expression.identifier.location);
                    builder.set_name(expression.identifier.name);
                    Ok((
                        ExpressionOperand::Identifier(builder.finish()),
                        location,
                        next,
                    ))
                }
            }
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::Boolean(boolean)),
                location,
            } => Ok((
                ExpressionOperand::LiteralBoolean(BooleanLiteral::new(location, boolean)),
                location,
                None,
            )),
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::Integer(integer)),
                location,
            } => Ok((
                ExpressionOperand::LiteralInteger(IntegerLiteral::new(location, integer)),
                location,
                None,
            )),
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::String(string)),
                location,
            } => Ok((
                ExpressionOperand::LiteralString(StringLiteral::new(location, string)),
                location,
                None,
            )),
            Token {
                lexeme: Lexeme::Keyword(keyword @ Keyword::SelfUppercase),
                location,
            } => {
                let mut builder = IdentifierBuilder::default();
                builder.set_location(location);
                builder.set_name(keyword.to_string());
                Ok((
                    ExpressionOperand::Identifier(builder.finish()),
                    location,
                    None,
                ))
            }
            Token { lexeme, location } => Err(Error::Syntax(
                SyntaxError::expected_expression_or_operand(location, lexeme),
            )),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::cell::RefCell;
//     use std::rc::Rc;
//
//     use super::Error;
//     use super::Parser;
//     use crate::lexical;
//     use crate::lexical::Lexeme;
//     use crate::lexical::Location;
//     use crate::lexical::Symbol;
//     use crate::lexical::Token;
//     use crate::lexical::TokenStream;
//     use crate::syntax::error::Error as SyntaxError;
//     use crate::syntax::tree::expression::tree::node::operand::Operand as ExpressionOperand;
//     use crate::syntax::tree::expression::tree::node::operator::Operator as ExpressionOperator;
//     use crate::syntax::tree::identifier::Identifier;
//     use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
//     use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
//     use crate::syntax::tree::literal::string::Literal as StringLiteral;
//
//     #[test]
//     fn ok_literal_boolean() {
//         let input = r#"true"#;
//
//         let expected = Ok((
//             Expression::new(
//                 Location::new(1, 1),
//                 vec![ExpressionElement::new(
//                     Location::new(1, 1),
//                     ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
//                         BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::True),
//                     )),
//                 )],
//             ),
//             None,
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_literal_integer() {
//         let input = r#"42"#;
//
//         let expected = Ok((
//             Expression::new(
//                 Location::new(1, 1),
//                 vec![ExpressionElement::new(
//                     Location::new(1, 1),
//                     ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                         IntegerLiteral::new(
//                             Location::new(1, 1),
//                             lexical::IntegerLiteral::new_decimal("42".to_owned()),
//                         ),
//                     )),
//                 )],
//             ),
//             None,
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_literal_string() {
//         let input = r#""description""#;
//
//         let expected = Ok((
//             Expression::new(
//                 Location::new(1, 1),
//                 vec![ExpressionElement::new(
//                     Location::new(1, 1),
//                     ExpressionObject::Operand(ExpressionOperand::LiteralString(
//                         StringLiteral::new(
//                             Location::new(1, 1),
//                             lexical::StringLiteral::new("description".to_owned()),
//                         ),
//                     )),
//                 )],
//             ),
//             None,
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_identifier() {
//         let input = r#"value"#;
//
//         let expected = Ok((
//             Expression::new(
//                 Location::new(1, 1),
//                 vec![ExpressionElement::new(
//                     Location::new(1, 1),
//                     ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
//                         Location::new(1, 1),
//                         "value".to_owned(),
//                     ))),
//                 )],
//             ),
//             Some(Token::new(Lexeme::Eof, Location::new(1, 6))),
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_alias_self() {
//         let input = r#"Self"#;
//
//         let expected = Ok((
//             Expression::new(
//                 Location::new(1, 1),
//                 vec![ExpressionElement::new(
//                     Location::new(1, 1),
//                     ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
//                         Location::new(1, 1),
//                         "Self".to_owned(),
//                     ))),
//                 )],
//             ),
//             None,
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn ok_parenthesized() {
//         let input = r#"(2 + 2)"#;
//
//         let expected = Ok((
//             Expression::new(
//                 Location::new(1, 1),
//                 vec![
//                     ExpressionElement::new(
//                         Location::new(1, 2),
//                         ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                             IntegerLiteral::new(
//                                 Location::new(1, 2),
//                                 lexical::IntegerLiteral::new_decimal("2".to_owned()),
//                             ),
//                         )),
//                     ),
//                     ExpressionElement::new(
//                         Location::new(1, 6),
//                         ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
//                             IntegerLiteral::new(
//                                 Location::new(1, 6),
//                                 lexical::IntegerLiteral::new_decimal("2".to_owned()),
//                             ),
//                         )),
//                     ),
//                     ExpressionElement::new(
//                         Location::new(1, 4),
//                         ExpressionObject::Operator(ExpressionOperator::Addition),
//                     ),
//                 ],
//             ),
//             None,
//         ));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
//
//     #[test]
//     fn error_expected() {
//         let input = r#"*"#;
//
//         let expected: Result<_, Error> =
//             Err(Error::Syntax(SyntaxError::expected_expression_or_operand(
//                 Location::new(1, 1),
//                 Lexeme::Symbol(Symbol::Asterisk),
//             )));
//
//         let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);
//
//         assert_eq!(result, expected);
//     }
// }
