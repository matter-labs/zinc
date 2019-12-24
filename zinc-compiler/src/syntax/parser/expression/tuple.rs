//!
//! The tuple expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionParser;
use crate::syntax::TupleExpressionBuilder;

#[derive(Debug, Clone, Copy)]
pub enum State {
    ParenthesisLeft,
    ExpressionOrParenthesisRight,
    CommaOrParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::ParenthesisLeft
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: TupleExpressionBuilder,
    next: Option<Token>,
}

///
/// The result can be either of:
/// 1. A unit type value
/// 2. A parenthesized expression
/// 3. A tuple of one or more elements
///
impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<Expression, Error> {
        loop {
            match self.state {
                State::ParenthesisLeft => {
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::ExpressionOrParenthesisRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["("],
                                lexeme,
                            )))
                        }
                    }
                }
                State::ExpressionOrParenthesisRight => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            return Ok(self.builder.finish());
                        }
                        token => {
                            let (expression, next) =
                                ExpressionParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_expression(expression);
                            self.state = State::CommaOrParenthesisRight;
                        }
                    }
                }
                State::CommaOrParenthesisRight => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => {
                            self.builder.set_comma();
                            self.state = State::ExpressionOrParenthesisRight;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ")"],
                                lexeme,
                            )));
                        }
                    }
                }
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
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::TupleExpression;

    #[test]
    fn ok_unit() {
        let input = r#"()"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 1),
                ExpressionObject::Operand(ExpressionOperand::Unit),
            )],
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_expression() {
        let input = r#"(1)"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 2),
                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(IntegerLiteral::new(
                    Location::new(1, 2),
                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                ))),
            )],
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_single() {
        let input = r#"(1,)"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 1),
                ExpressionObject::Operand(ExpressionOperand::Tuple(TupleExpression::new(
                    Location::new(1, 1),
                    vec![Expression::new(
                        Location::new(1, 2),
                        vec![ExpressionElement::new(
                            Location::new(1, 2),
                            ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                IntegerLiteral::new(
                                    Location::new(1, 2),
                                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                                ),
                            )),
                        )],
                    )],
                ))),
            )],
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"(1, 2, 3)"#;

        let expected = Ok(Expression::new(
            Location::new(1, 1),
            vec![ExpressionElement::new(
                Location::new(1, 1),
                ExpressionObject::Operand(ExpressionOperand::Tuple(TupleExpression::new(
                    Location::new(1, 1),
                    vec![
                        Expression::new(
                            Location::new(1, 2),
                            vec![ExpressionElement::new(
                                Location::new(1, 2),
                                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::new(1, 2),
                                        lexical::IntegerLiteral::new_decimal("1".to_owned()),
                                    ),
                                )),
                            )],
                        ),
                        Expression::new(
                            Location::new(1, 5),
                            vec![ExpressionElement::new(
                                Location::new(1, 5),
                                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::new(1, 5),
                                        lexical::IntegerLiteral::new_decimal("2".to_owned()),
                                    ),
                                )),
                            )],
                        ),
                        Expression::new(
                            Location::new(1, 8),
                            vec![ExpressionElement::new(
                                Location::new(1, 8),
                                ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                                    IntegerLiteral::new(
                                        Location::new(1, 8),
                                        lexical::IntegerLiteral::new_decimal("3".to_owned()),
                                    ),
                                )),
                            )],
                        ),
                    ],
                ))),
            )],
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
