//!
//! The assignment operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperator;
use crate::syntax::RangeOperandParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    RangeFirstOperand,
    RangeOperator,
    RangeSecondOperand,
}

impl Default for State {
    fn default() -> Self {
        State::RangeFirstOperand
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
                State::RangeFirstOperand => {
                    let (expression, next) =
                        RangeOperandParser::default().parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.set_location_if_unset(expression.location);
                    self.builder.extend_with_expression(expression);
                    self.state = State::RangeOperator;
                }
                State::RangeOperator => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleDot),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::Range));
                            self.state = State::RangeSecondOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleDotEquals),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::RangeInclusive));
                            self.state = State::RangeSecondOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
                    }
                }
                State::RangeSecondOperand => {
                    let (expression, token) = RangeOperandParser::default().parse(stream, None)?;
                    self.builder.extend_with_expression(expression);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    return Ok((self.builder.finish(), token));
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
    use crate::syntax::IntegerLiteral;

    #[test]
    fn ok_range() {
        let input = r#"0 .. 9"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 1),
                                lexical::IntegerLiteral::new_decimal("0".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 6),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 6),
                                lexical::IntegerLiteral::new_decimal("9".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 3),
                        ExpressionObject::Operator(ExpressionOperator::Range),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 7))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_range_inclusive() {
        let input = r#"0 ..= 9"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 1),
                                lexical::IntegerLiteral::new_decimal("0".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 7),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 7),
                                lexical::IntegerLiteral::new_decimal("9".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 3),
                        ExpressionObject::Operator(ExpressionOperator::RangeInclusive),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
