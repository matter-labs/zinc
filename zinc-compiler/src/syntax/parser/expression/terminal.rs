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
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::expression::array::Parser as ArrayExpressionParser;
use crate::syntax::parser::expression::block::Parser as BlockExpressionParser;
use crate::syntax::parser::expression::conditional::Parser as ConditionalExpressionParser;
use crate::syntax::parser::expression::r#match::Parser as MatchExpressionParser;
use crate::syntax::parser::expression::structure::Parser as StructureExpressionParser;
use crate::syntax::parser::expression::tuple::Parser as TupleExpressionParser;
use crate::syntax::tree::expression::builder::Builder as ExpressionBuilder;
use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
use crate::syntax::tree::expression::Expression;
use crate::syntax::tree::identifier::builder::Builder as IdentifierBuilder;
use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
use crate::syntax::tree::literal::string::Literal as StringLiteral;

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
        match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                ..
            } => {
                self.builder.set_location(token.location);
                let expression = TupleExpressionParser::default().parse(stream, Some(token))?;
                self.builder.extend_with_expression(expression);
                Ok((self.builder.finish(), None))
            }
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                ..
            } => {
                self.builder.set_location(token.location);
                let block = BlockExpressionParser::default().parse(stream, Some(token))?;
                self.builder
                    .push_operand(block.location, ExpressionOperand::Block(block));
                Ok((self.builder.finish(), None))
            }
            token
            @
            Token {
                lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                ..
            } => {
                self.builder.set_location(token.location);
                let array = ArrayExpressionParser::default().parse(stream, Some(token))?;
                self.builder
                    .push_operand(array.location, ExpressionOperand::Array(array));
                Ok((self.builder.finish(), None))
            }
            token
            @
            Token {
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
            token
            @
            Token {
                lexeme: Lexeme::Keyword(Keyword::Match),
                ..
            } => {
                self.builder.set_location(token.location);
                let expression = MatchExpressionParser::default().parse(stream, Some(token))?;
                self.builder
                    .push_operand(expression.location, ExpressionOperand::Match(expression));
                Ok((self.builder.finish(), None))
            }
            token
            @
            Token {
                lexeme: Lexeme::Identifier(..),
                ..
            } => {
                self.builder.set_location(token.location);
                let (expression, next) =
                    StructureExpressionParser::default().parse(stream, Some(token))?;
                if expression.is_struct {
                    self.builder.push_operand(
                        expression.location,
                        ExpressionOperand::Structure(expression),
                    );
                } else {
                    let mut builder = IdentifierBuilder::default();
                    builder.set_location(expression.identifier.location);
                    builder.set_name(expression.identifier.name);
                    self.builder.push_operand(
                        expression.location,
                        ExpressionOperand::Identifier(builder.finish()),
                    );
                }
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
                lexeme: Lexeme::Keyword(keyword @ Keyword::SelfUppercase),
                location,
            } => {
                self.builder.set_location(location);
                let mut builder = IdentifierBuilder::default();
                builder.set_location(location);
                builder.set_name(keyword.to_string());
                self.builder
                    .push_operand(location, ExpressionOperand::Identifier(builder.finish()));
                Ok((self.builder.finish(), None))
            }
            Token { lexeme, location } => Err(Error::Syntax(
                SyntaxError::expected_expression_or_operand(location, lexeme),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Error;
    use super::Parser;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::literal::string::Literal as StringLiteral;

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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
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
            Some(Token::new(Lexeme::Eof, Location::new(1, 6))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_alias_self() {
        let input = r#"Self"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![ExpressionElement::new(
                    Location::new(1, 1),
                    ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                        Location::new(1, 1),
                        "Self".to_owned(),
                    ))),
                )],
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected() {
        let input = r#"*"#;

        let expected: Result<_, Error> =
            Err(Error::Syntax(SyntaxError::expected_expression_or_operand(
                Location::new(1, 1),
                Lexeme::Symbol(Symbol::Asterisk),
            )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
