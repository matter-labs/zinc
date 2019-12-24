//!
//! The array/tuple/structure access operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionListParser;
use crate::syntax::ExpressionOperand;
use crate::syntax::ExpressionOperator;
use crate::syntax::ExpressionParser;
use crate::syntax::IntegerLiteral;
use crate::syntax::MemberIntegerBuilder;
use crate::syntax::MemberStringBuilder;
use crate::syntax::PathOperandParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    PathOperand,
    AccessOrCallOrEnd,
    IndexExpression,
    BracketSquareRight,
    FieldDescriptor,
    ArgumentList,
    ParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::PathOperand
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ExpressionBuilder,
    operator: Option<(Location, ExpressionOperator)>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Expression, Option<Token>), Error> {
        loop {
            match self.state {
                State::PathOperand => {
                    let (expression, next) =
                        PathOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.set_location(expression.location);
                    self.builder.extend_with_expression(expression);
                    self.state = State::AccessOrCallOrEnd;
                }
                State::AccessOrCallOrEnd => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Index));
                            self.state = State::IndexExpression;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Call));
                            self.state = State::ArgumentList;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Dot),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Field));
                            self.state = State::FieldDescriptor;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::IndexExpression => {
                    let (expression, next) =
                        ExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.next = next;
                    self.builder.extend_with_expression(expression);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::BracketSquareRight;
                }
                State::BracketSquareRight => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => {
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["]"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::FieldDescriptor => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme:
                                Lexeme::Literal(lexical::Literal::Integer(
                                    literal @ lexical::IntegerLiteral::Decimal { .. },
                                )),
                            location,
                        } => {
                            let mut builder = MemberIntegerBuilder::default();
                            builder.set_location(location);
                            builder.set_literal(IntegerLiteral::new(location, literal));
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::MemberInteger(builder.finish()),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let mut builder = MemberStringBuilder::default();
                            builder.set_location(location);
                            builder.set_name(identifier.name);
                            self.builder.push_operand(
                                location,
                                ExpressionOperand::MemberString(builder.finish()),
                            );
                            if let Some((location, operator)) = self.operator.take() {
                                self.builder.push_operator(location, operator);
                            }
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{decimal}", "{identifier}"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::ArgumentList => {
                    let (expression_list, next) =
                        ExpressionListParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operand(
                            location,
                            ExpressionOperand::ExpressionList(expression_list),
                        );
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::ParenthesisRight;
                }
                State::ParenthesisRight => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => {
                            self.state = State::AccessOrCallOrEnd;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![")"],
                                lexeme,
                            )))
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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::MemberInteger;
    use crate::syntax::MemberString;

    #[test]
    fn ok() {
        let input = r#"array[42].25.value"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::Identifier(Identifier::new(
                            Location::new(1, 1),
                            "array".to_owned(),
                        ))),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 7),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 7),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 6),
                        ExpressionObject::Operator(ExpressionOperator::Index),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 11),
                        ExpressionObject::Operand(ExpressionOperand::MemberInteger(
                            MemberInteger::new(
                                Location::new(1, 11),
                                IntegerLiteral::new(
                                    Location::new(1, 11),
                                    lexical::IntegerLiteral::new_decimal("25".to_owned()),
                                ),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 10),
                        ExpressionObject::Operator(ExpressionOperator::Field),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 14),
                        ExpressionObject::Operand(ExpressionOperand::MemberString(
                            MemberString::new(Location::new(1, 14), "value".to_owned()),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 13),
                        ExpressionObject::Operator(ExpressionOperator::Field),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 19))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
