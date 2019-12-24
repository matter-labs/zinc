//!
//! The terminal operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ArrayExpressionParser;
use crate::syntax::BlockExpressionParser;
use crate::syntax::BooleanLiteral;
use crate::syntax::ConditionalExpressionParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperand;
use crate::syntax::IdentifierBuilder;
use crate::syntax::IntegerLiteral;
use crate::syntax::MatchExpressionParser;
use crate::syntax::StringLiteral;
use crate::syntax::StructureExpressionParser;
use crate::syntax::TupleExpressionParser;

#[derive(Default)]
pub struct Parser {
    builder: ExpressionBuilder,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Expression, Option<Token>), Error> {
        match match initial.take() {
            Some(next) => next,
            None => stream.borrow_mut().next()?,
        } {
            token @ Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => {
                self.builder.set_location(token.location);
                let expression = TupleExpressionParser::default().parse(stream, Some(token))?;
                self.builder.extend_with_expression(expression);
                Ok((self.builder.finish(), None))
            }
            token @ Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                ..
            } => {
                self.builder.set_location(token.location);
                let block = BlockExpressionParser::default().parse(stream, Some(token))?;
                self.builder
                    .push_operand(block.location, ExpressionOperand::Block(block));
                Ok((self.builder.finish(), None))
            }
            token @ Token {
                lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                ..
            } => {
                self.builder.set_location(token.location);
                let array = ArrayExpressionParser::default().parse(stream, Some(token))?;
                self.builder
                    .push_operand(array.location, ExpressionOperand::Array(array));
                Ok((self.builder.finish(), None))
            }
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::If),
                ..
            } => {
                self.builder.set_location(token.location);
                let (expression, next) =
                    ConditionalExpressionParser::default().parse(stream, Some(token))?;
                self.builder.push_operand(
                    expression.location,
                    ExpressionOperand::Conditional(expression),
                );
                Ok((self.builder.finish(), next))
            }
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Match),
                ..
            } => {
                self.builder.set_location(token.location);
                let expression = MatchExpressionParser::default().parse(stream, Some(token))?;
                self.builder
                    .push_operand(expression.location, ExpressionOperand::Match(expression));
                Ok((self.builder.finish(), None))
            }
            token @ Token {
                lexeme: Lexeme::Keyword(Keyword::Struct),
                ..
            } => {
                self.builder.set_location(token.location);
                let (expression, next) =
                    StructureExpressionParser::default().parse(stream, Some(token))?;
                self.builder.push_operand(
                    expression.location,
                    ExpressionOperand::Structure(expression),
                );
                Ok((self.builder.finish(), next))
            }
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::Boolean(boolean)),
                location,
            } => {
                self.builder.set_location(location);
                self.builder.push_operand(
                    location,
                    ExpressionOperand::LiteralBoolean(BooleanLiteral::new(location, boolean)),
                );
                Ok((self.builder.finish(), None))
            }
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::Integer(integer)),
                location,
            } => {
                self.builder.set_location(location);
                self.builder.push_operand(
                    location,
                    ExpressionOperand::LiteralInteger(IntegerLiteral::new(location, integer)),
                );
                Ok((self.builder.finish(), None))
            }
            Token {
                lexeme: Lexeme::Literal(lexical::Literal::String(string)),
                location,
            } => {
                self.builder.set_location(location);
                self.builder.push_operand(
                    location,
                    ExpressionOperand::LiteralString(StringLiteral::new(location, string)),
                );
                Ok((self.builder.finish(), None))
            }
            Token {
                lexeme: Lexeme::Identifier(identifier),
                location,
            } => {
                self.builder.set_location(location);
                let mut identifier_builder = IdentifierBuilder::default();
                identifier_builder.set_location(location);
                identifier_builder.set_name(identifier.name);
                self.builder.push_operand(
                    location,
                    ExpressionOperand::Identifier(identifier_builder.finish()),
                );
                Ok((self.builder.finish(), None))
            }
            Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                location,
                vec![
                    "(",
                    "{",
                    "[",
                    "if",
                    "match",
                    "struct",
                    "{literal}",
                    "{identifier}",
                ],
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
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::StringLiteral;
    use crate::syntax::{BooleanLiteral, ExpressionOperator};

    #[test]
    fn ok_literal_boolean() {
        let input = r#"true"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                        BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::True),
                    )),
                )],
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
    fn ok_literal_integer() {
        let input = r#"42"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 1),
                            lexical::IntegerLiteral::new_decimal("42".to_owned()),
                        ),
                    )),
                )],
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
    fn ok_literal_string() {
        let input = r#""description""#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::LiteralString(
                        StringLiteral::new(
                            Location::new(1, 1),
                            lexical::StringLiteral::new("description".to_owned()),
                        ),
                    )),
                )],
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
    fn ok_identifier() {
        let input = r#"value"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "value".to_owned(),
                    ))),
                )],
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
    fn ok_parenthesized() {
        let input = r#"(2 + 2)"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 2),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 2),
                                lexical::IntegerLiteral::new_decimal("2".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 6),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 6),
                                lexical::IntegerLiteral::new_decimal("2".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 4),
                        ExpressionObject::Operator(ExpressionOperator::Addition),
                    ),
                ],
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
