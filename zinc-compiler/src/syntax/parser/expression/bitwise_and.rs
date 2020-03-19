//!
//! The bitwise AND operand parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Location;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::parser::expression::bitwise_shift::Parser as BitwiseShiftOperandParser;
use crate::syntax::tree::expression::builder::Builder as ExpressionBuilder;
use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
use crate::syntax::tree::expression::Expression;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BitwiseShiftOperand,
    BitwiseShiftOperator,
}

impl Default for State {
    fn default() -> Self {
        State::BitwiseShiftOperand
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
                State::BitwiseShiftOperand => {
                    let (expression, next) = BitwiseShiftOperandParser::default()
                        .parse(stream.clone(), initial.take())?;
                    self.next = next;
                    self.builder.set_location_if_unset(expression.location);
                    self.builder.extend_with_expression(expression);
                    if let Some((location, operator)) = self.operator.take() {
                        self.builder.push_operator(location, operator);
                    }
                    self.state = State::BitwiseShiftOperator;
                }
                State::BitwiseShiftOperator => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleLesser),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::BitwiseShiftLeft));
                            self.state = State::BitwiseShiftOperand;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleGreater),
                            location,
                        } => {
                            self.operator = Some((location, ExpressionOperator::BitwiseShiftRight));
                            self.state = State::BitwiseShiftOperand;
                        }
                        token => return Ok((self.builder.finish(), Some(token))),
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
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok() {
        let input = r#"42 << 2"#;

        let expected = Ok((
            Expression::new(
                Location::new(1, 1),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 1),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 1),
                                lexical::IntegerLiteral::new_decimal("42".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 7),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 7),
                                lexical::IntegerLiteral::new_decimal("2".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 4),
                        ExpressionObject::Operator(ExpressionOperator::BitwiseShiftLeft),
                    ),
                ],
            ),
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
