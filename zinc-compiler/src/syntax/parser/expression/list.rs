//!
//! The expression (function argument) list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::syntax::ExpressionParser;

#[derive(Default)]
pub struct Parser {
    expressions: Vec<Expression>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Vec<Expression>, Option<Token>), Error> {
        loop {
            match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                token
                @
                Token {
                    lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                    ..
                } => return Ok((self.expressions, Some(token))),
                token
                @
                Token {
                    lexeme: Lexeme::Eof,
                    ..
                } => return Ok((self.expressions, Some(token))),
                token => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    self.expressions.push(expression);
                }
            }

            match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => continue,
                token => return Ok((self.expressions, Some(token))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::BooleanLiteral;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_single() {
        let input = r#"true || false"#;

        let expected = Ok((
            vec![Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::True),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 9),
                        ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 9),
                                lexical::BooleanLiteral::False,
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 6),
                        ExpressionObject::Operator(ExpressionOperator::Or),
                    ),
                ],
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 14))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"true || false,"#;

        let expected = Ok((
            vec![Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(Location::new(1, 1), lexical::BooleanLiteral::True),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 9),
                        ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                            BooleanLiteral::new(
                                Location::new(1, 9),
                                lexical::BooleanLiteral::False,
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 6),
                        ExpressionObject::Operator(ExpressionOperator::Or),
                    ),
                ],
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 15))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"true || false, 42 as field"#;

        let expected = Ok((
            vec![
                Expression::new(
                    Location::new(1, 1),
                    vec![
                        ExpressionElement::new(
                            Location::new(1, 1),
                            ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                                BooleanLiteral::new(
                                    Location::new(1, 1),
                                    lexical::BooleanLiteral::True,
                                ),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 9),
                            ExpressionObject::Operand(ExpressionOperand::LiteralBoolean(
                                BooleanLiteral::new(
                                    Location::new(1, 9),
                                    lexical::BooleanLiteral::False,
                                ),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 6),
                            ExpressionObject::Operator(ExpressionOperator::Or),
                        ),
                    ],
                ),
                Expression::new(
                    Location::new(1, 16),
                    vec![
                        ExpressionElement::new(
                            Location::new(1, 16),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 16),
                                    lexical::IntegerLiteral::new_decimal("42".to_owned()),
                                ),
                            )),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 22),
                            ExpressionObject::Operand(ExpressionOperand::Type(Type::new(
                                Location::new(1, 22),
                                TypeVariant::field(),
                            ))),
                        ),
                        ExpressionElement::new(
                            Location::new(1, 19),
                            ExpressionObject::Operator(ExpressionOperator::Casting),
                        ),
                    ],
                ),
            ],
            Some(Token::new(Lexeme::Eof, Location::new(1, 27))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            Vec::<Expression>::new(),
            Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
